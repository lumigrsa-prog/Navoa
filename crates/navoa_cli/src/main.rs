use navoa_codegen::Codegen;
use navoa_vm::Vm;
use navoa_parser::Parser;
use navoa_lexer::{Lexer, Language};

fn main() {
    println!("=== Navoa CLI ===");
    
    let codigo_exemplo = "imprimir 42;";
    let mut lexer = Lexer::new(codigo_exemplo, Language::Pt);
    
    match lexer.tokenize() {
        Ok(tokens) => {
            let mut parser = Parser::new(&tokens);
            
            match parser.parse() {
                Ok(statements) => {
                    let codegen = Codegen::new();
                    let _gerado = codegen.gerar(&statements);
                    
                    let mut vm = Vm::new();
                    vm.executar(statements);
                    
                    println!("Execução e geração de código concluídas com sucesso!");
                }
                Err(e) => {
                    eprintln!("Erro de parsing: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Erro de análise léxica: {}", e);
        }
    }
}
