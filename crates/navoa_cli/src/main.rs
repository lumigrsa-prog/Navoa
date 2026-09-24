use navoa_lexer::{Lexer, Token};
use navoa_parser::Parser;
use navoa_vm::Vm;

fn main() {
    println!("=== Navoa CLI ===");

    let codigo_exemplo = "imprimir \"Olá, Navoa!\";";
    let mut lexer = Lexer::novo(codigo_exemplo);
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
            println!("{}", vm.obter_saida());
            println!("Execução concluída com sucesso!");
        }
        Err(e) => {
            eprintln!("Erro de sintaxe: {:?}", e);
        }
    }
}
