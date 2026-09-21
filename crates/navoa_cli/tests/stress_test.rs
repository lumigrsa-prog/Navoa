use navoa_bytecode::{BinaryOp, BytecodeProgram, Instruction};
use navoa_vm::VM;

#[test]
fn test_vm_stress_execution() {
    // Empilhar e somar 1000 valores sequenciais
    let mut instructions = vec![Instruction::PushInt(0)];
    for i in 1..1000 {
        instructions.push(Instruction::PushInt(i));
        instructions.push(Instruction::Binary(BinaryOp::Add));
    }
    instructions.push(Instruction::Pop);

    let program = BytecodeProgram { instructions };

    let mut vm = VM::new();
    let result = vm.run(&program);
    assert!(result.is_ok());
}
