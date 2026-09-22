use navoa_ast::{Expr, Statement};

pub struct Codegen;

impl Codegen {
    pub fn new() -> Self {
        Codegen
    }

    pub fn gerar(&self, statements: &[Statement]) -> String {
        let mut codigo = String::new();
        for stmt in statements {
            match stmt {
                Statement::Imprimir(expr) => {
                    codigo.push_str(&format!("print({});\n", self.gerar_expr(expr)));
                }
                Statement::Atribuir(nome, expr) => {
                    codigo.push_str(&format!("let {} = {};\n", nome, self.gerar_expr(expr)));
                }
            }
        }
        codigo
    }

    fn gerar_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Numero(n) => n.to_string(),
            Expr::Texto(t) => format!("\"{}\"", t),
            Expr::Identificador(id) => id.clone(),
        }
    }
}
