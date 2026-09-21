#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Print(String),
    Expr(String),
    If {
        condition: String,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    While {
        condition: String,
        body: Vec<Statement>,
    },
}
