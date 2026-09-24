use navoa_ast::Statement;
use navoa_vm::Vm;

#[test]
fn test_vm_stress_execution() {
    let statements: Vec<Statement> = vec![];
    let mut vm = Vm::new();
    for _ in 0..1000 {
        vm.executar(statements.clone());
    }
}
