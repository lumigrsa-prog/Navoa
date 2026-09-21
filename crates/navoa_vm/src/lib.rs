use navoa_parser::{Statement, Expr, BinaryOp};
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

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<(), String> {
        for stmt in statements {
            self.execute_stmt(&stmt)?;
        }
        Ok(())
    }

    fn execute_stmt(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::VarDecl { name, value } => {
                let val = self.evaluate_expr(value)?;
                self.env.define(name.clone(), val);
            }
            Statement::Print(expr) => {
                let val = self.evaluate_expr(expr)?;
                println!("{}", val);
            }
            Statement::Assign { name, value } => {
                let val = self.evaluate_expr(value)?;
                self.env.assign(name, val)?;
            }
            Statement::While { condition, body } => {
                while self.is_truthy(&self.evaluate_expr(condition)?) {
                    self.env.push_scope(); // Novo scope para o bloco while
                    for s in body {
                        if let Err(e) = self.execute_stmt(s) {
                            self.env.pop_scope();
                            return Err(e);
                        }
                    }
                    self.env.pop_scope();
                }
            }
            Statement::If { condition, then_branch, else_branch } => {
                if self.is_truthy(&self.evaluate_expr(condition)?) {
                    self.env.push_scope();
                    for s in then_branch {
                        if let Err(e) = self.execute_stmt(s) {
                            self.env.pop_scope();
                            return Err(e);
                        }
                    }
                    self.env.pop_scope();
                } else if let Some(else_b) = else_branch {
                    self.env.push_scope();
                    for s in else_b {
                        if let Err(e) = self.execute_stmt(s) {
                            self.env.pop_scope();
                            return Err(e);
                        }
                    }
                    self.env.pop_scope();
                }
            }
            Statement::Expr(expr) => {
                self.evaluate_expr(expr)?;
            }
            Statement::FunctionDecl { .. } => {
                return Err("Declaração de funções suportada pelo parser, mas VM precisa de implementação.".to_string());
            }
            Statement::Return(_) => {
                return Err("Return suportado pelo parser, mas VM precisa de implementação.".to_string());
            }
        }
        Ok(())
    }

    fn evaluate_expr(&self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::Str(s.clone())),
            Expr::Variable(name) => self.env.get(name),
            Expr::Binary { left, op, right } => {
                let l_val = self.evaluate_expr(left)?;
                let r_val = self.evaluate_expr(right)?;

                match (l_val, op, r_val) {
                    // Operações Numéricas
                    (Value::Number(l), BinaryOp::Add, Value::Number(r)) => Ok(Value::Number(l + r)),
                    (Value::Number(l), BinaryOp::Sub, Value::Number(r)) => Ok(Value::Number(l - r)),
                    (Value::Number(l), BinaryOp::Mul, Value::Number(r)) => Ok(Value::Number(l * r)),
                    (Value::Number(l), BinaryOp::Div, Value::Number(r)) => {
                        if r == 0.0 {
                            Err("Divisão por zero.".to_string())
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    }
                    // Concatenação de Strings
                    (Value::Str(l), BinaryOp::Add, Value::Str(r)) => Ok(Value::Str(format!("{}{}", l, r))),
                    
                    // Comparadores Numéricos (Retornam 1.0 para verdadeiro e 0.0 para falso)
                    (Value::Number(l), BinaryOp::GreaterThan, Value::Number(r)) => Ok(Value::Number(if l > r { 1.0 } else { 0.0 })),
                    (Value::Number(l), BinaryOp::LessThan, Value::Number(r)) => Ok(Value::Number(if l < r { 1.0 } else { 0.0 })),
                    (Value::Number(l), BinaryOp::Equal, Value::Number(r)) => Ok(Value::Number(if l == r { 1.0 } else { 0.0 })),

                    _ => Err(format!("Operação inválida entre tipos para o operador '{:?}'", op)),
                }
            }
            Expr::Array(_) => Err("Arrays ainda não implementados na VM".to_string()),
            Expr::Index { .. } => Err("Indexação ainda não implementada na VM".to_string()),
            Expr::Call { .. } => Err("Chamadas de função ainda não implementadas na VM".to_string()),
        }
    }

    fn is_truthy(&self, val: &Value) -> bool {
        match val {
            Value::Number(n) => *n != 0.0,
            Value::Str(s) => !s.is_empty(),
        }
    }
}
