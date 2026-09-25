use navoa_core::lexer::Lexer;
use navoa_core::parser::Parser;

#[test]
fn test_parser_stress() {
    let code = "[1, 2, 3, 4, 5]";
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    assert!(parser.parse().is_ok());
}
