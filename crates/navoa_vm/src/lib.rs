use navoa_ast::{Statement, Expr};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Valor {
    Numero(f64),
    Texto(String),
    Nulo,
}

pub struct Vm {
    env: HashMap<String, Valor>,
    buffer_saida: String,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
            buffer_saida: String::new(),
        }
    }

    pub fn executar(&mut self, statements: Vec<Statement>) {
        for stmt in statements {
            self.executar_statement(stmt);
        }
    }

    pub fn obter_saida(&self) -> String {
        if self.buffer_saida.is_empty() {
            "Execução concluída sem saídas.".to_string()
        } else {
            self.buffer_saida.clone()
        }
    }

    fn executar_statement(&mut self, stmt: Statement) {
        match stmt {
            Statement::Imprimir(expr) => {
                let val = self.eval_expr(expr);
                let output = match val {
                    Valor::Numero(n) => format!("{}", n),
                    Valor::Texto(s) => s,
                    Valor::Nulo => "nulo".to_string(),
                };
                println!("{}", output);
                self.buffer_saida.push_str(&output);
                self.buffer_saida.push('\n');
            }
            Statement::Atribuir(nome, expr) => {
                let val = self.eval_expr(expr);
                self.env.insert(nome, val);
            }
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Valor {
        match expr {
            Expr::Numero(n) => Valor::Numero(n),
            Expr::Texto(s) => Valor::Texto(s),
            Expr::Identificador(nome) => self.env.get(&nome).cloned().unwrap_or(Valor::Nulo),
        }
    }
}
