use navoa_lexer::{Language, Lexer, Token};
use navoa_ast::{AstStmt, Expr};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    _lang: Language,
}

impl Parser {
    pub fn new(source: &str, lang: Language) -> Result<Self, String> {
        let mut lexer = Lexer::new(source, lang.clone());
        let mut tokens = Vec::new();
        loop {
            let token = lexer.next_token();
            let is_eof = matches!(token, Token::Eof);
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        Ok(Parser {
            tokens,
            current: 0,
            _lang: lang,
        })
    }

    pub fn parse_program(&mut self) -> Result<Vec<AstStmt>, String> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if self.check(&Token::Semicolon) {
                self.advance();
                continue;
            }
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<AstStmt, String> {
        if self.match_token(&[Token::Var]) {
            return self.var_declaration();
        }
        if self.match_token(&[Token::Print]) {
            return self.print_statement();
        }
        if self.match_token(&[Token::While]) {
            return self.while_statement();
        }
        if self.match_token(&[Token::LBrace]) {
            let stmts = self.block_statements()?;
            return Ok(AstStmt::Block(stmts));
        }

        if let Token::Identifier(name) = self.peek().clone() {
            if self.check_next(&Token::Equal) {
                self.advance();
                self.advance();
                let value = self.expression()?;
                self.consume_semicolon()?;
                return Ok(AstStmt::Assign { name, value });
            }
        }

        Err(format!(
            "Erro de sintaxe perto de '{:?}'",
            self.peek()
        ))
    }

    fn var_declaration(&mut self) -> Result<AstStmt, String> {
        let name = match self.advance().clone() {
            Token::Identifier(n) => n,
            other => return Err(format!("Esperado nome de variável, encontrado '{:?}'", other)),
        };

        if !self.match_token(&[Token::Equal]) {
            return Err("Esperado '=' após nome da variável".to_string());
        }

        let value = self.expression()?;
        self.consume_semicolon()?;
        Ok(AstStmt::VarDecl { name, value })
    }

    fn print_statement(&mut self) -> Result<AstStmt, String> {
        let value = self.expression()?;
        self.consume_semicolon()?;
        Ok(AstStmt::Print(value))
    }

    fn while_statement(&mut self) -> Result<AstStmt, String> {
        self.consume(&Token::LParen, "Esperado '(' após 'enquanto/while'.")?;
        let condition = self.expression()?;
        self.consume(&Token::RParen, "Esperado ')' após condição do ciclo.")?;

        let body = self.statement()?;
        Ok(AstStmt::While {
            condition,
            body: Box::new(body),
        })
    }

    fn block_statements(&mut self) -> Result<Vec<AstStmt>, String> {
        let mut stmts = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            if self.check(&Token::Semicolon) {
                self.advance();
                continue;
            }
            stmts.push(self.statement()?);
        }
        self.consume(&Token::RBrace, "Esperado '}' no final do bloco.")?;
        Ok(stmts)
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.match_token(&[Token::EqualEqual, Token::BangEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_token(&[Token::Greater, Token::GreaterEqual, Token::Less, Token::LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_token(&[Token::Plus, Token::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;

        while self.match_token(&[Token::Star, Token::Slash]) {
            let operator = self.previous().clone();
            let right = self.primary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[Token::Number(0.0)]) {
            if let Token::Number(n) = self.previous() {
                return Ok(Expr::Number(*n));
            }
        }
        if self.match_token(&[Token::String(String::new())]) {
            if let Token::String(s) = self.previous() {
                return Ok(Expr::String(s.clone()));
            }
        }
        if let Token::Identifier(name) = self.peek().clone() {
            self.advance();
            return Ok(Expr::Variable(name));
        }
        if self.match_token(&[Token::LParen]) {
            let expr = self.expression()?;
            self.consume(&Token::RParen, "Esperado ')' após expressão.")?;
            return Ok(Expr::Binary {
                left: Box::new(expr),
                operator: Token::LParen,
                right: Box::new(Expr::Number(0.0)),
            });
        }

        Err(format!("Expressão inválida encontrada: '{:?}'", self.peek()))
    }

    fn match_token(&mut self, types: &[Token]) -> bool {
        for t in types {
            if self.check_discriminant(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(self.peek()) == std::mem::discriminant(token)
    }

    fn check_next(&self, token: &Token) -> bool {
        if self.current + 1 >= self.tokens.len() {
            return false;
        }
        std::mem::discriminant(&self.tokens[self.current + 1]) == std::mem::discriminant(token)
    }

    fn check_discriminant(&self, token: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(self.peek()) == std::mem::discriminant(token)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token: &Token, message: &str) -> Result<&Token, String> {
        if self.check(token) {
            Ok(self.advance())
        } else {
            Err(format!("{} (Encontrado: '{:?}')", message, self.peek()))
        }
    }

    fn consume_semicolon(&mut self) -> Result<(), String> {
        if self.check(&Token::Semicolon) {
            self.advance();
        }
        Ok(())
    }
}
