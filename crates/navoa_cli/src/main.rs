use navoa_core::lexer::Lexer;
use navoa_core::parser::Parser;

fn main() {
    let code = "[10, 20, 30]";
    println!("Código fonte: {}", code);

    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => println!("AST gerada com sucesso:\n{:#?}", ast),
        Err(err) => eprintln!("Erro de compilação: {}", err),
    }
}

