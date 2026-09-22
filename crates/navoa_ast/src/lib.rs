#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Numero(f64),
    Texto(String),
    Identificador(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Imprimir(Expr),
    Atribuir(String, Expr),
}
