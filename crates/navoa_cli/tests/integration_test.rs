use navoa_lexer::Lexer;
use navoa_parser::Parser;
use navoa_vm::VM;

#[test]
fn test_funcoes_e_retorno() {
    let codigo = "
        funcao somar(a, b) {
            retornar a
        }
        var res = somar(10, 20)
        imprimir res
    ";

    let lexer = Lexer::novo(codigo);
    let mut parser = Parser::novo(lexer);
    let programa = parser.parse_programa();

    let mut vm = VM::nova();
    vm.executar(&programa);
    assert_eq!(vm.obter_saida(), "10");
}
