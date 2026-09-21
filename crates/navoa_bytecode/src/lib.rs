#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    PushInt(i64),
    PushFloat(f64),
    PushString(String),
    PushBool(bool),
    LoadVar(String),
    StoreVar(String),
    Binary(BinaryOp),
    Print,
    Jump(usize),
    JumpIfFalse(usize),
    Call(String, usize),
    Return,
    Pop,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BytecodeProgram {
    pub instructions: Vec<Instruction>,
}

impl BytecodeProgram {
    pub fn new() -> Self {
        BytecodeProgram {
            instructions: Vec::new(),
        }
    }

    pub fn emit(&mut self, instruction: Instruction) -> usize {
        self.instructions.push(instruction);
        self.instructions.len() - 1
    }
}
