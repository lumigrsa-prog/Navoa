use navoa_lexer::Lexer;
use navoa_parser::Parser;
use navoa_vm::VM;

#[test]
fn test_integracao_basico() {
    let codigo = "var x = 10\nimprimir x";
    let lexer = Lexer::novo(codigo);
    let mut parser = Parser::novo(lexer);
    let programa = parser.parse_programa();

    let mut vm = VM::nova();
    vm.executar(&programa);
    assert_eq!(vm.obter_saida(), "10");
}
