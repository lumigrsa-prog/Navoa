use crate::ast::Expr;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Nil,
}

pub struct VM;

impl VM {
    pub fn new() -> Self {
        Self
    }

    pub fn interpret(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Array(elements) => {
                let mut vals = Vec::new();
                for elem in elements {
                    vals.push(self.interpret(elem)?);
                }
                Ok(Value::Array(vals))
            }
            Expr::IndexAccess { target, index } => {
                let target_val = self.interpret(target)?;
                let index_val = self.interpret(index)?;

                match (target_val, index_val) {
                    (Value::Array(arr), Value::Number(i)) => {
                        let idx = i as usize;
                        arr.get(idx)
                            .cloned()
                            .ok_or_else(|| format!("Índice fora dos limites: {}", idx))
                    }
                    _ => Err("Acesso a índice inválido".to_string()),
                }
            }
            _ => Err("Expressão não suportada na VM".to_string()),
        }
    }
}
