use std::collections::HashMap;
use navoa_ast::{Expr, Statement};

#[derive(Debug, Clone, PartialEq)]
pub enum Valor {
    Numero(f64),
    Texto(String),
    Nulo,
}

impl std::fmt::Display for Valor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Valor::Numero(n) => write!(f, "{}", n),
            Valor::Texto(t) => write!(f, "{}", t),
            Valor::Nulo => write!(f, "nulo"),
        }
    }
}

pub struct Vm {
    ambiente: HashMap<String, Valor>,
}

impl Vm {
    pub fn new() -> Self {
        Vm {
            ambiente: HashMap::new(),
        }
    }

    pub fn executar(&mut self, statements: Vec<Statement>) {
        for stmt in statements {
            self.executar_statement(&stmt);
        }
    }

    fn executar_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Imprimir(expr) => {
                let valor = self.avaliar_expressao(expr);
                println!("{}", valor);
            }
            Statement::Atribuir(nome, expr) => {
                let valor = self.avaliar_expressao(expr);
                self.ambiente.insert(nome.clone(), valor);
            }
        }
    }

    fn avaliar_expressao(&self, expr: &Expr) -> Valor {
        match expr {
            Expr::Numero(n) => Valor::Numero(*n),
            Expr::Texto(t) => Valor::Texto(t.clone()),
            Expr::Identificador(nome) => {
                self.ambiente.get(nome).cloned().unwrap_or(Valor::Nulo)
            }
        }
    }
}
