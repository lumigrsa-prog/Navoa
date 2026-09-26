use navoa_core::NavoaEngine;
use tiny_http::{Server, Response, Header};

fn main() {
    let port = 8080;
    let server = Server::http(format!("0.0.0.0:{}", port)).unwrap();
    println!("✨ Servidor Gráfico ativo em http://localhost:8080");

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
        ("🏆 CONCLUÍDO", "🎉 <b>AVENTURA FINALIZADA COM SUCESSO!</b><br>O Navoa está livre nas ruas de Lisboa. Todos os desafios de programação foram superados com distinção.")
    } else {
        match chapter {
            1 => ("Cap. 1: O Despertar", "🛏️ Usa <b>inspecionar quarto</b> para procurar pistas."),
            2 => ("Cap. 2: A Máquina", "☕ Usa <b>inspecionar maquina</b>. Vais precisar de aprender condições (SE)."),
            3 => ("Cap. 3: O Cofre", "🔐 Usa <b>inspecionar cofre</b>. Vais precisar de usar um CICLO para rodar a fechadura."),
            4 => ("Cap. 4: A Análise", "📊 Usa <b>inspecionar sistema</b>. Avalia os riscos."),
            5 => ("Cap. 5: A Saída", "🚪 Usa <b>inspecionar porta</b>. É a última barreira."),
            _ => ("Erro", "Erro.")
        }
    };

    let controls = if completed {
        r#"<div class="input-row">
            <button class="btn-close" onclick="closeSession()">Fechar Aventura</button>
        </div>"#.to_string()
    } else {
        r#"<div class="input-row">
            <input type="text" id="cmdInput" placeholder="Comando..." autofocus autocomplete="off">
            <button onclick="sendCmd()">Executar</button>
        </div>"#.to_string()
    };

    format!(r#"
<!DOCTYPE html>
<html lang="pt">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Navoa OS - {}</title>
    <style>
        body {{ background: #0f111a; color: #fff; font-family: monospace; display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; margin: 0; }}
        .container {{ width: 90%; max-width: 600px; background: #1a1c29; border-radius: 8px; padding: 24px; border: 1px solid #333; box-shadow: 0 10px 25px rgba(0,0,0,0.5); }}
        .header {{ margin-bottom: 20px; border-bottom: 1px solid #333; padding-bottom: 10px; display: flex; justify-content: space-between; align-items: center; }}
        .badge {{ background: #22c55e; color: #000; padding: 4px 8px; border-radius: 4px; font-weight: bold; font-size: 12px; }}
        .story-text {{ background: #11131d; padding: 12px; border-left: 4px solid #6366f1; margin-bottom: 15px; font-size: 14px; line-height: 1.5; }}
        .terminal-box {{ background: #000; border-radius: 4px; padding: 12px; text-align: left; height: 150px; overflow-y: auto; color: #4ade80; margin-bottom: 15px; border: 1px solid #222; }}
        .input-row {{ display: flex; gap: 10px; }}
        input {{ flex: 1; background: #000; border: 1px solid #4ade80; border-radius: 4px; padding: 12px; color: #4ade80; font-family: monospace; outline: none; }}
        button {{ background: #4ade80; color: #000; font-weight: bold; border: none; padding: 0 20px; border-radius: 4px; cursor: pointer; font-family: monospace; }}
        button:hover {{ opacity: 0.9; }}
        .btn-close {{ background: #ef4444; color: #fff; width: 100%; padding: 14px; font-size: 15px; font-weight: bold; border-radius: 4px; cursor: pointer; text-align: center; border: none; }}
        .btn-close:hover {{ background: #dc2626; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h2>{}</h2>
            {}
        </div>
        <div class="story-text">{}</div>
        <div class="terminal-box" id="term">> Terminal Navoa online...<br>{}</div>
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
            term.innerHTML += `<br><span style="color:#fff">> ${{cmd}}</span>`;

            fetch('/api/exec', {{ method: 'POST', body: cmd }})
            .then(res => res.json())
            .then(data => {{
                term.innerHTML += `<br>${{data.output.replace(/\\n/g, '<br>')}}`;
                term.scrollTop = term.scrollHeight;
                input.value = '';

                if (data.completed || data.chapter !== currentChapter) {{
                    setTimeout(() => location.reload(), 1500);
                }}
            }});
        }}

        function closeSession() {{
            window.close();
            alert('Aventura concluída! Podes fechar este separador do browser.');
        }}
    </script>
</body>
</html>
"#, 
    title,
    title,
    if completed { r#"<span class="badge">FINALIZADO</span>"# } else { "" },
    desc,
    if completed { "> 🏆 ESTADO: SISTEMA TOTALMENTE OPERACIONAL." } else { "> Dica: Se não sabes o que fazer, inspeciona as coisas." },
    controls,
    chapter
)
}
