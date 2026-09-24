#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use navoa_parser::Parser;
use navoa_vm::Vm;

#[tauri::command]
fn executar_navoa(codigo: String) -> Result<String, String> {
    let mut parser = Parser::novo(&codigo);
    match parser.analisar() {
        Ok(statements) => {
            let mut vm = Vm::new();
            vm.executar(statements);
            Ok("Execução concluída com sucesso!".to_string())
        }
        Err(e) => Err(format!("Erro de compilação: {:?}", e)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![executar_navoa])
        .run(tauri::generate_context!())
        .expect("Erro ao iniciar o Navoa Studio");
}
