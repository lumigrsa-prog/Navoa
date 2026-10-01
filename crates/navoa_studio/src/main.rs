// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use navoa_lexer::Lexer;
use navoa_parser::Parser;
use navoa_vm::VM;

#[tauri::command]
fn run_code(code: String) -> Result<String, String> {
    // Inicializa o teu Lexer
    let lexer = Lexer::novo(&code);
    
    // Inicializa o Parser passando o Lexer
    let mut parser = Parser::novo(lexer);
    
    // Roda a tua lógica de parsing (retorna a struct Programa)
    let programa = parser.parse_programa();
    
    // Inicializa e corre a VM
    let mut vm = VM::nova();
    vm.executar(&programa);
    
    // Retorna a saída capturada pela VM
    Ok(vm.obter_saida())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_code])
        .run(tauri::generate_context!())
        .expect("erro ao executar a aplicação tauri");
}
