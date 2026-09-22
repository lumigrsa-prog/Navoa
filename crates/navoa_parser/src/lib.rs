pub use navoa_ast::Statement;
use navoa_lexer::Token;

pub struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, position: 0 }
    }

    #[allow(dead_code)]
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let statements = Vec::new();
        Ok(statements)
    }
}
