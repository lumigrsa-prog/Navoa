// Trecho completo e corrigido do parser para a verificação de fim de ficheiro e tokens
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        // Usa a capitalização exata da variante do token (Eof em vez de EOF, se for o caso)
        self.check(&Token::Eof) || self.current >= self.tokens.len()
    }

    fn check(&self, token_type: &Token) -> bool {
        if self.is_out_of_bounds() {
            return false;
        }
        &self.tokens[self.current] == token_type
    }

    fn is_out_of_bounds(&self) -> bool {
        self.current >= self.tokens.len()
    }

    // Restantes métodos do parser...
}
