use navoa_ast::AstStmt;

pub struct Analyzer {}

impl Analyzer {
    pub fn new() -> Self {
        Analyzer {}
    }

    pub fn analyze(&mut self, ast: Vec<AstStmt>) -> Result<Vec<AstStmt>, String> {
        Ok(ast)
    }
}
