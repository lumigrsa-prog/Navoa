use navoa_ast::{AstStmt, Expr};
use navoa_lexer::Token;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Str(String),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::Str(s) => write!(f, "{}", s),
        }
    }
}

pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    pub fn get(&self, name: &str) -> Result<Value, String> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Ok(val.clone());
            }
        }
        Err(format!("Variável não definida: '{}'", name))
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }
        Err(format!("Variável não definida para atribuição: '{}'", name))
    }
}

pub struct VM {
    env: Environment,
}

impl VM {
    pub fn new() -> Self {
        VM {
            env: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<AstStmt>) -> Result<(), String> {
        for stmt in statements {
            self.execute_stmt(&stmt)?;
        }
        Ok(())
    }

    fn execute_stmt(&mut self, stmt: &AstStmt) -> Result<(), String> {
        match stmt {
            AstStmt::VarDecl { name, value } => {
                let val = self.evaluate_expr(value)?;
                self.env.define(name.clone(), val);
            }
            AstStmt::Print(expr) => {
                let val = self.evaluate_expr(expr)?;
                println!("{}", val);
            }
            AstStmt::Assign { name, value } => {
                let val = self.evaluate_expr(value)?;
                self.env.assign(name, val)?;
            }
            AstStmt::Block(stmts) => {
                self.env.push_scope();
                for s in stmts {
                    if let Err(e) = self.execute_stmt(s) {
                        self.env.pop_scope();
                        return Err(e);
                    }
                }
                self.env.pop_scope();
            }
            AstStmt::While { condition, body } => {
                while self.is_truthy(&self.evaluate_expr(condition)?) {
                    self.execute_stmt(body)?;
                }
            }
        }
        Ok(())
    }

    fn evaluate_expr(&self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::Str(s.clone())),
            Expr::Variable(name) => self.env.get(name),
            Expr::Binary { left, operator, right } => {
                let l_val = self.evaluate_expr(left)?;
                let r_val = self.evaluate_expr(right)?;

                match (l_val, operator, r_val) {
                    (Value::Number(l), Token::Plus, Value::Number(r)) => Ok(Value::Number(l + r)),
                    (Value::Number(l), Token::Minus, Value::Number(r)) => Ok(Value::Number(l - r)),
                    (Value::Number(l), Token::Star, Value::Number(r)) => Ok(Value::Number(l * r)),
                    (Value::Number(l), Token::Slash, Value::Number(r)) => {
                        if r == 0.0 {
                            Err("Divisão por zero.".to_string())
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    }
                    (Value::Str(l), Token::Plus, Value::Str(r)) => Ok(Value::Str(format!("{}{}", l, r))),
                    
                    // Comparadores numéricos
                    (Value::Number(l), Token::Greater, Value::Number(r)) => Ok(Value::Number(if l > r { 1.0 } else { 0.0 })),
                    (Value::Number(l), Token::GreaterEqual, Value::Number(r)) => Ok(Value::Number(if l >= r { 1.0 } else { 0.0 })),
                    (Value::Number(l), Token::Less, Value::Number(r)) => Ok(Value::Number(if l < r { 1.0 } else { 0.0 })),
                    (Value::Number(l), Token::LessEqual, Value::Number(r)) => Ok(Value::Number(if l <= r { 1.0 } else { 0.0 })),
                    (Value::Number(l), Token::EqualEqual, Value::Number(r)) => Ok(Value::Number(if l == r { 1.0 } else { 0.0 })),
                    (Value::Number(l), Token::BangEqual, Value::Number(r)) => Ok(Value::Number(if l != r { 1.0 } else { 0.0 })),

                    _ => Err(format!("Operação inválida entre tipos para o operador '{:?}'", operator)),
                }
            }
        }
    }

    fn is_truthy(&self, val: &Value) -> bool {
        match val {
            Value::Number(n) => *n != 0.0,
            Value::Str(s) => !s.is_empty(),
        }
    }
}
