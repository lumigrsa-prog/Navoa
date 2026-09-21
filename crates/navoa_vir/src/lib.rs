#[derive(Debug, Clone, PartialEq)]
pub enum VirOp {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VirExpr {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Var(String),
    Binary {
        left: Box<VirExpr>,
        op: VirOp,
        right: Box<VirExpr>,
    },
    Call {
        name: String,
        args: Vec<VirExpr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum VirStmt {
    VarDecl {
        name: String,
        value: VirExpr,
    },
    Assignment {
        name: String,
        value: VirExpr,
    },
    Print(VirExpr),
    Expr(VirExpr),
    If {
        condition: VirExpr,
        then_branch: Vec<VirStmt>,
        else_branch: Option<Vec<VirStmt>>,
    },
    While {
        condition: VirExpr,
        body: Vec<VirStmt>,
    },
    FunctionDecl {
        name: String,
        params: Vec<String>,
        body: Vec<VirStmt>,
    },
    Return(Option<VirExpr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VirProgram {
    pub statements: Vec<VirStmt>,
}
