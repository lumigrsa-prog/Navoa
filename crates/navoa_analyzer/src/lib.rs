use navoa_vir::VirStatement;

pub struct Analyzer;

impl Analyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&mut self, statements: Vec<VirStatement>) -> Result<Vec<VirStatement>, String> {
        Ok(statements)
    }
}
