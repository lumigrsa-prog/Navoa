use navoa_core::lexer::Lexer;
use navoa_core::parser::Parser;
use navoa_core::vm::{Value, VM};

#[test]
fn test_full_pipeline_array() {
    let code = "[10, 20, 30]";
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Erro ao gerar AST");

    let mut vm = VM::new();
    let result = vm.interpret(&ast).expect("Erro ao executar na VM");

    assert_eq!(
        result,
        Value::Array(vec![
            Value::Number(10.0),
            Value::Number(20.0),
            Value::Number(30.0),
        ])
    );
}
