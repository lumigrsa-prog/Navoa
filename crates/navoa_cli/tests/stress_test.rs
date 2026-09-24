use navoa_lexer::Lexer;
use navoa_parser::Parser;
use navoa_vm::VM;

#[test]
fn test_stress_execucao() {
    let mut codigo = String::new();
    for _ in 0..100 {
        codigo.push_str("imprimir 1\n");
    }

    let lexer = Lexer::novo(&codigo);
    let mut parser = Parser::novo(lexer);
    let programa = parser.parse_programa();

    let mut vm = VM::nova();
    vm.executar(&programa);
    assert!(!vm.obter_saida().is_empty());
}
