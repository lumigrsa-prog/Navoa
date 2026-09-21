#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp { Add, Sub, Mul, Div, Equal, LessThan, GreaterThan }

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    String(String),
    Variable(String),
    Array(Vec<Expr>),
    Index { target: Box<Expr>, index: Box<Expr> },
    Call { name: String, args: Vec<Expr> },
    Binary { left: Box<Expr>, op: BinaryOp, right: Box<Expr> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VarDecl { name: String, value: Expr },
    Assign { name: String, value: Expr },
    FunctionDecl { name: String, params: Vec<String>, body: Vec<Statement> },
    Return(Expr),
    Print(Expr),
    Expr(Expr),
    If { condition: Expr, then_branch: Vec<Statement>, else_branch: Option<Vec<Statement>> },
    While { condition: Expr, body: Vec<Statement> },
}
