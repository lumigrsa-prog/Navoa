use navoa_ast::Statement;
use navoa_vm::Vm;

#[test]
fn test_vm_basic_execution() {
    let statements: Vec<Statement> = vec![];
    let mut vm = Vm::new();
    vm.executar(statements);
}
