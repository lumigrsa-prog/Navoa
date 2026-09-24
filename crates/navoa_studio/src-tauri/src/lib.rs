use navoa_codegen::Codegen;
use navoa_lexer::Lexer;
use navoa_parser::Parser;
use navoa_vm::VM;
use serde::Serialize;

#[derive(Serialize)]
pub struct ResultadoExecucao {
    pub saida: String,
    pub js_transpilado: String,
}

#[tauri::command]
fn executar_navoa(codigo: String) -> ResultadoExecucao {
    let lexer = Lexer::novo(&codigo);
    let mut parser = Parser::novo(lexer);
    let programa = parser.parse_programa();

    let mut vm = VM::nova();
    vm.executar(&programa);
    let saida = vm.obter_saida();

    let js_transpilado = Codegen::gerar_js(&programa);

    ResultadoExecucao {
        saida,
        js_transpilado,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![executar_navoa])
        .run(tauri::generate_context!())
        .expect("Erro ao iniciar o Navoa Studio");
}
