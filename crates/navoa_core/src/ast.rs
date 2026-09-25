#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Identifier(String),
    Array(Vec<Expr>),
    IndexAccess {
        target: Box<Expr>,
        index: Box<Expr>,
    },
}
