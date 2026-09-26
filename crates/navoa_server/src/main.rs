use navoa_core::NavoaEngine;
use tiny_http::{Server, Response, Header};

fn main() {
    let port = 8080;
    let server = Server::http(format!("0.0.0.0:{}", port)).unwrap();
    println!("🕵️  Navoa OS - Terminal Retro Noir ativo em http://localhost:{}", port);

    for mut request in server.incoming_requests() {
        let url = request.url().to_string();
        let mut engine = NavoaEngine::new();

        if url.starts_with("/api/exec") {
            let mut content = String::new();
            let mut reader = request.as_reader();
            let _ = std::io::Read::read_to_string(&mut reader, &mut content);

            let output = engine.execute(&content);
            let json = format!(
                r#"{{"output":"{}","chapter":{},"completed":{}}}"#, 
                output.replace('\n', "\\n").replace('"', "\\\""), 
                engine.config.chapter,
                engine.config.challenge_completed
            );
            
            let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
            let _ = request.respond(Response::from_string(json).with_header(header));
        } else {
            let html = get_html_ui(engine.config.chapter, engine.config.challenge_completed);
            let header = Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap();
            let _ = request.respond(Response::from_string(html).with_header(header));
        }
    }
}

fn get_html_ui(chapter: u32, completed: bool) -> String {
    let (title, desc) = if completed {
        ("🏆 INVESTIGAÇÃO CONCLUÍDA", "PARABÉNS, DETETIVE. O Navoa ultrapassou todos os obstáculos e libertou-se na noite de Lisboa.")
    } else {
        match chapter {
            1 => ("CAP. 1: O DESPERTAR", "🛏️ Quarto escuro. Inspeciona o quarto com 'inspecionar quarto' para restaurar a energia."),
            2 => ("CAP. 2: O CAFÉ NOIR", "☕ Inspeciona a máquina com 'inspecionar maquina' para entender a condição SE."),
            3 => ("CAP. 3: O COFRE SECRETO", "🔐 Inspeciona o cofre com 'inspecionar cofre' para desativar a tranca com REPETIR."),
            4 => ("CAP. 4: ANÁLISE DE RISCO", "📊 Inspeciona o sistema com 'inspecionar sistema' para avaliar as ameaças."),
            5 => ("CAP. 5: A FUGA DE LISBOA", "🚪 Inspeciona a porta com 'inspecionar porta' para abrir a passagem final."),
            _ => ("ERRO DE SISTEMA", "Ficheiro corrompido.")
        }
    };

    let controls = if completed {
        r#"<div class="input-row">
            <button class="btn-close" onclick="closeSession()">[ X ] FECHAR SESSÃO</button>
        </div>"#.to_string()
    } else {
        r#"<div class="input-row">
            <span class="prompt-symbol">&gt;</span>
            <input type="text" id="cmdInput" placeholder="digita um comando..." autofocus autocomplete="off">
            <button onclick="sendCmd()">EXECUTAR</button>
        </div>"#.to_string()
    };

    format!(r#"
<!DOCTYPE html>
<html lang="pt">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Navoa OS - Retro Noir</title>
    <style>
        @import url('https://fonts.googleapis.com/css2?family=VT323&display=swap');

        * {{
            box-sizing: border-box;
        }}

        body {{
            background-color: #050507;
            color: #ffb000;
            font-family: 'VT323', 'Courier New', monospace;
            display: flex;
            align-items: center;
            justify-content: center;
            min-height: 100vh;
            margin: 0;
            padding: 15px;
            overflow: hidden;
            text-shadow: 0 0 4px rgba(255, 176, 0, 0.7);
        }}

        /* Efeito de Vidro CRT e Ecrã Curvo */
        .crt-monitor {{
            width: 100%;
            max-width: 680px;
            background: #0d0c07;
            border: 12px solid #1c1a14;
            border-radius: 20px;
            padding: 25px;
            box-shadow: 0 0 50px rgba(0, 0, 0, 0.9), inset 0 0 100px rgba(0, 0, 0, 0.8), 0 0 15px rgba(255, 176, 0, 0.15);
            position: relative;
            animation: crtFlicker 0.15s infinite alternate;
        }}

        /* Overlay de Linhas de Varrimento (Scanlines) */
        .crt-monitor::before {{
            content: " ";
            display: block;
            position: absolute;
            top: 0; left: 0; bottom: 0; right: 0;
            background: linear-gradient(rgba(18, 16, 11, 0) 50%, rgba(0, 0, 0, 0.4) 50%);
            background-size: 100% 4px;
            z-index: 10;
            pointer-events: none;
            opacity: 0.7;
            border-radius: 8px;
        }}

        .header {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            border-bottom: 2px dashed #ffb000;
            padding-bottom: 10px;
            margin-bottom: 15px;
            font-size: 22px;
            letter-spacing: 1px;
        }}

        .badge {{
            background: #ffb000;
            color: #0d0c07;
            padding: 2px 8px;
            font-weight: bold;
            font-size: 18px;
        }}

        .story-text {{
            background: rgba(255, 176, 0, 0.05);
            border: 1px solid rgba(255, 176, 0, 0.3);
            padding: 12px 15px;
            margin-bottom: 15px;
            font-size: 20px;
            line-height: 1.4;
        }}

        .terminal-box {{
            background: #070604;
            border: 1px solid #ffb000;
            padding: 12px;
            height: 180px;
            overflow-y: auto;
            font-size: 19px;
            line-height: 1.3;
            margin-bottom: 15px;
            color: #ffc84d;
        }}

        .input-row {{
            display: flex;
            gap: 10px;
            align-items: center;
        }}

        .prompt-symbol {{
            font-size: 24px;
            font-weight: bold;
        }}

        input {{
            flex: 1;
            background: #000;
            border: 1px solid #ffb000;
            padding: 10px;
            color: #ffb000;
            font-family: 'VT323', monospace;
            font-size: 20px;
            outline: none;
            text-shadow: 0 0 5px rgba(255, 176, 0, 0.8);
        }}

        input:focus {{
            box-shadow: 0 0 8px rgba(255, 176, 0, 0.6);
        }}

        button {{
            background: #ffb000;
            color: #0d0c07;
            border: none;
            padding: 10px 18px;
            font-family: 'VT323', monospace;
            font-size: 20px;
            font-weight: bold;
            cursor: pointer;
            letter-spacing: 1px;
        }}

        button:hover {{
            background: #ffe082;
            box-shadow: 0 0 10px rgba(255, 176, 0, 0.8);
        }}

        .btn-close {{
            background: #d32f2f;
            color: #fff;
            width: 100%;
            padding: 12px;
            font-size: 22px;
            border: 1px solid #ff6666;
            cursor: pointer;
            text-align: center;
        }}

        .btn-close:hover {{
            background: #b71c1c;
            box-shadow: 0 0 10px rgba(211, 47, 47, 0.8);
        }}

        /* Animação de tremor CRT discreto */
        @keyframes crtFlicker {{
            0% {{ opacity: 0.97; }}
            100% {{ opacity: 1; }}
        }}

        /* Scrollbar Retro */
        ::-webkit-scrollbar {{
            width: 8px;
        }}
        ::-webkit-scrollbar-track {{
            background: #070604;
        }}
        ::-webkit-scrollbar-thumb {{
            background: #ffb000;
        }}
    </style>
</head>
<body>
    <div class="crt-monitor">
        <div class="header">
            <span>SYS.NAVOA v1.0 // {}</span>
            {}
        </div>
        
        <div class="story-text">{}</div>
        
        <div class="terminal-box" id="term">
            > SISTEMA OPERACIONAL NAVOA // TERMINAL DE INVESTIGAÇÃO<br>
            {}
        </div>

        {}
    </div>

    <script>
        const currentChapter = {};
        const input = document.getElementById('cmdInput');
        if (input) {{
            input.addEventListener('keypress', e => {{ if (e.key === 'Enter') sendCmd(); }});
        }}

        function sendCmd() {{
            if (!input) return;
            const cmd = input.value.trim();
            if (!cmd) return;
            const term = document.getElementById('term');
            term.innerHTML += `<br><span style="color:#ffffff">&gt; ${{cmd}}</span>`;

            fetch('/api/exec', {{ method: 'POST', body: cmd }})
            .then(res => res.json())
            .then(data => {{
                term.innerHTML += `<br>${{data.output.replace(/\\n/g, '<br>')}}`;
                term.scrollTop = term.scrollHeight;
                input.value = '';

                if (data.completed || data.chapter !== currentChapter) {{
                    setTimeout(() => location.reload(), 1400);
                }}
            }});
        }}

        function closeSession() {{
            alert('Sessão encerrada com sucesso.');
            window.close();
        }}
    </script>
</body>
</html>
"#, 
    title,
    if completed { r#"<span class="badge">RESOLVIDO</span>"# } else { "" },
    desc,
    if completed { "> REGISTO: CASO FECHADO COM SUCESSO." } else { "> DICA: Digita 'inspecionar <objeto>' para investigar o cenário." },
    controls,
    chapter
)
}
