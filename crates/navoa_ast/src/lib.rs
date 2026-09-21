use navoa_lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    String(String),
    Variable(String),
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstStmt {
    VarDecl { name: String, value: Expr },
    Print(Expr),
    Assign { name: String, value: Expr },
    Block(Vec<AstStmt>),
    While { condition: Expr, body: Box<AstStmt> },
}
