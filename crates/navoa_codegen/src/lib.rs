use navoa_bytecode::{BinaryOp, BytecodeProgram, Instruction};
use navoa_vir::{VirExpr, VirOp, VirProgram, VirStmt};

pub struct CodeGen {
    program: BytecodeProgram,
}

impl CodeGen {
    pub fn new() -> Self {
        CodeGen {
            program: BytecodeProgram::new(),
        }
    }

    pub fn generate(&mut self, stmts: &[VirStmt]) -> Result<BytecodeProgram, String> {
        for stmt in stmts {
            self.gen_stmt(stmt)?;
        }
        Ok(self.program.clone())
    }

    pub fn generate_program(&mut self, vir: &VirProgram) -> Result<BytecodeProgram, String> {
        self.generate(&vir.statements)
    }

    fn gen_stmt(&mut self, stmt: &VirStmt) -> Result<(), String> {
        match stmt {
            VirStmt::VarDecl { name, value } => {
                self.gen_expr(value)?;
                self.program.emit(Instruction::StoreVar(name.clone()));
            }
            VirStmt::Assignment { name, value } => {
                self.gen_expr(value)?;
                self.program.emit(Instruction::StoreVar(name.clone()));
            }
            VirStmt::Print(expr) => {
                self.gen_expr(expr)?;
                self.program.emit(Instruction::Print);
            }
            VirStmt::Expr(expr) => {
                self.gen_expr(expr)?;
                self.program.emit(Instruction::Pop);
            }
            VirStmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.gen_expr(condition)?;
                let jump_false_idx = self.program.emit(Instruction::JumpIfFalse(0));

                for s in then_branch {
                    self.gen_stmt(s)?;
                }

                if let Some(else_b) = else_branch {
                    let jump_end_idx = self.program.emit(Instruction::Jump(0));
                    let else_target = self.program.instructions.len();
                    self.program.instructions[jump_false_idx] =
                        Instruction::JumpIfFalse(else_target);

                    for s in else_b {
                        self.gen_stmt(s)?;
                    }
                    let end_target = self.program.instructions.len();
                    self.program.instructions[jump_end_idx] = Instruction::Jump(end_target);
                } else {
                    let end_target = self.program.instructions.len();
                    self.program.instructions[jump_false_idx] =
                        Instruction::JumpIfFalse(end_target);
                }
            }
            VirStmt::While { condition, body } => {
                let loop_start = self.program.instructions.len();
                self.gen_expr(condition)?;
                let jump_false_idx = self.program.emit(Instruction::JumpIfFalse(0));

                for s in body {
                    self.gen_stmt(s)?;
                }

                self.program.emit(Instruction::Jump(loop_start));
                let loop_end = self.program.instructions.len();
                self.program.instructions[jump_false_idx] = Instruction::JumpIfFalse(loop_end);
            }
            VirStmt::FunctionDecl { .. } => {}
            VirStmt::Return(value) => {
                if let Some(expr) = value {
                    self.gen_expr(expr)?;
                }
                self.program.emit(Instruction::Return);
            }
        }
        Ok(())
    }

    fn gen_expr(&mut self, expr: &VirExpr) -> Result<(), String> {
        match expr {
            VirExpr::Int(n) => {
                self.program.emit(Instruction::PushInt(*n));
            }
            VirExpr::Float(f) => {
                self.program.emit(Instruction::PushFloat(*f));
            }
            VirExpr::String(s) => {
                self.program.emit(Instruction::PushString(s.clone()));
            }
            VirExpr::Bool(b) => {
                self.program.emit(Instruction::PushBool(*b));
            }
            VirExpr::Var(name) => {
                self.program.emit(Instruction::LoadVar(name.clone()));
            }
            VirExpr::Binary { left, op, right } => {
                self.gen_expr(left)?;
                self.gen_expr(right)?;
                self.program.emit(Instruction::Binary(map_vir_op(op)));
            }
            VirExpr::Call { name, args } => {
                for arg in args {
                    self.gen_expr(arg)?;
                }
                self.program.emit(Instruction::Call(name.clone(), args.len()));
            }
        }
        Ok(())
    }
}

fn map_vir_op(op: &VirOp) -> BinaryOp {
    match op {
        VirOp::Add => BinaryOp::Add,
        VirOp::Sub => BinaryOp::Sub,
        VirOp::Mul => BinaryOp::Mul,
        VirOp::Div => BinaryOp::Div,
        VirOp::Equal => BinaryOp::Equal,
        VirOp::NotEqual => BinaryOp::NotEqual,
        VirOp::LessThan => BinaryOp::LessThan,
        VirOp::LessEqual => BinaryOp::LessEqual,
        VirOp::GreaterThan => BinaryOp::GreaterThan,
        VirOp::GreaterEqual => BinaryOp::GreaterEqual,
        VirOp::And => BinaryOp::And,
        VirOp::Or => BinaryOp::Or,
    }
}
