#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use navoa_lexer::{Lexer, Token};
use navoa_parser::Parser;
use navoa_vm::Vm;

#[tauri::command]
fn executar_navoa(codigo: String) -> Result<String, String> {
    let mut lexer = Lexer::novo(&codigo);
    let mut tokens = Vec::new();

    loop {
        let tok = lexer.proximo_token();
        if tok == Token::EOF {
            break;
        }
        tokens.push(tok);
    }

    let mut parser = Parser::new(&tokens);
    match parser.parse() {
        Ok(statements) => {
            let mut vm = Vm::new();
            vm.executar(statements);
            Ok(vm.obter_saida())
        }
        Err(e) => Err(format!("Erro de sintaxe: {:?}", e)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![executar_navoa])
        .run(tauri::generate_context!())
        .expect("Erro ao iniciar o Navoa Studio");
}
