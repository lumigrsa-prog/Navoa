use navoa_bytecode::BytecodeProgram;
use navoa_vm::VM;

#[test]
fn test_vm_basic_execution() {
    // BytecodeProgram recebe diretamente a lista de instruções geradas pelo codegen
    let program = BytecodeProgram {
        instructions: vec![],
    };

    let mut vm = VM::new();
    let result = vm.run(&program);
    assert!(result.is_ok());
}
