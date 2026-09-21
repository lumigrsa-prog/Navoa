use navoa_lexer::{Lexer, Token};
use navoa_vir::{VirExpr, VirOp, VirStatement};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(source: &str) -> Result<Self, String> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        Ok(Parser { tokens, current: 0 })
    }

    pub fn parse_program(&mut self) -> Result<Vec<VirStatement>, String> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<VirStatement, String> {
        if self.match_token(&[Token::Imprimir]) {
            self.parse_print_statement()
        } else if self.match_token(&[Token::Se]) {
            self.parse_if_statement()
        } else if self.match_token(&[Token::Enquanto]) {
            self.parse_while_statement()
        } else if self.match_token(&[Token::Retornar]) {
            self.parse_return_statement()
        } else if self.match_token(&[Token::Funcao]) {
            self.parse_function_decl()
        } else if self.check_assignment() {
            self.parse_assignment()
        } else {
            let expr = self.parse_expression()?;
            Ok(VirStatement::Expr(expr))
        }
    }

    fn parse_function_decl(&mut self) -> Result<VirStatement, String> {
        let name = match self.advance() {
            Some(Token::Identifier(n)) => n.clone(),
            token => return Err(format!("Esperado nome da função. Encontrado: {:?}", token)),
        };

        if !self.match_token(&[Token::LParen]) {
            return Err("Esperado '(' após nome da função.".to_string());
        }

        let mut params = Vec::new();
        if !self.check(&Token::RParen) {
            loop {
                match self.advance() {
                    Some(Token::Identifier(p)) => params.push(p.clone()),
                    token => return Err(format!("Esperado parâmetro. Encontrado: {:?}", token)),
                }
                if self.match_token(&[Token::Comma]) {
                    continue;
                }
                break;
            }
        }

        if !self.match_token(&[Token::RParen]) {
            return Err("Esperado ')' após parâmetros.".to_string());
        }

        if !self.match_token(&[Token::LBrace]) {
            return Err("Esperado '{' no corpo da função.".to_string());
        }

        let body = self.parse_block()?;

        Ok(VirStatement::FunctionDecl { name, params, body })
    }

    fn check_assignment(&self) -> bool {
        if let Some(Token::Identifier(_)) = self.peek() {
            if let Some(Token::Equal) = self.peek_next() {
                return true;
            }
        }
        false
    }

    fn parse_assignment(&mut self) -> Result<VirStatement, String> {
        let name = match self.advance() {
            Some(Token::Identifier(n)) => n.clone(),
            _ => unreachable!(),
        };

        self.advance();

        let value = self.parse_expression()?;

        Ok(VirStatement::Assignment { name, value })
    }

    fn parse_print_statement(&mut self) -> Result<VirStatement, String> {
        let has_paren = self.match_token(&[Token::LParen]);
        let expr = self.parse_expression()?;
        if has_paren {
            if !self.match_token(&[Token::RParen]) {
                return Err("Esperado ')' após expressão de impressão.".to_string());
            }
        }
        Ok(VirStatement::Print(expr))
    }

    fn parse_if_statement(&mut self) -> Result<VirStatement, String> {
        let has_paren = self.match_token(&[Token::LParen]);
        let condition = self.parse_expression()?;
        if has_paren {
            if !self.match_token(&[Token::RParen]) {
                return Err("Esperado ')' após condição do 'se'.".to_string());
            }
        }

        if !self.match_token(&[Token::LBrace]) {
            return Err("Esperado '{' após condição do 'se'.".to_string());
        }

        let then_branch = self.parse_block()?;

        let else_branch = if self.match_token(&[Token::Senao]) {
            if self.match_token(&[Token::Se]) {
                Some(vec![self.parse_if_statement()?])
            } else if self.match_token(&[Token::LBrace]) {
                Some(self.parse_block()?)
            } else {
                return Err("Esperado '{' ou 'se' após 'senao'.".to_string());
            }
        } else {
            None
        };

        Ok(VirStatement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_while_statement(&mut self) -> Result<VirStatement, String> {
        let has_paren = self.match_token(&[Token::LParen]);
        let condition = self.parse_expression()?;
        if has_paren {
            if !self.match_token(&[Token::RParen]) {
                return Err("Esperado ')' após condição do 'enquanto'.".to_string());
            }
        }

        if !self.match_token(&[Token::LBrace]) {
            return Err("Esperado '{' no bloco do 'enquanto'.".to_string());
        }

        let body = self.parse_block()?;

        Ok(VirStatement::While { condition, body })
    }

    fn parse_return_statement(&mut self) -> Result<VirStatement, String> {
        if self.check(&Token::RBrace) || self.is_at_end() {
            Ok(VirStatement::Return(None))
        } else {
            let value = self.parse_expression()?;
            Ok(VirStatement::Return(Some(value)))
        }
    }

    fn parse_block(&mut self) -> Result<Vec<VirStatement>, String> {
        let mut statements = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        if !self.match_token(&[Token::RBrace]) {
            return Err("Esperado '}' no final do bloco.".to_string());
        }

        Ok(statements)
    }

    fn parse_expression(&mut self) -> Result<VirExpr, String> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<VirExpr, String> {
        let mut expr = self.parse_logical_and()?;

        while self.match_token(&[Token::Or]) {
            let right = self.parse_logical_and()?;
            expr = VirExpr::Binary {
                left: Box::new(expr),
                op: VirOp::Or,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> Result<VirExpr, String> {
        let mut expr = self.parse_equality()?;

        while self.match_token(&[Token::And]) {
            let right = self.parse_equality()?;
            expr = VirExpr::Binary {
                left: Box::new(expr),
                op: VirOp::And,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<VirExpr, String> {
        let mut expr = self.parse_comparison()?;

        while let Some(op) = self.match_equality_op() {
            let right = self.parse_comparison()?;
            expr = VirExpr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn match_equality_op(&mut self) -> Option<VirOp> {
        if self.match_token(&[Token::EqualEqual]) {
            Some(VirOp::Equal)
        } else if self.match_token(&[Token::NotEqual]) {
            Some(VirOp::NotEqual)
        } else {
            None
        }
    }

    fn parse_comparison(&mut self) -> Result<VirExpr, String> {
        let mut expr = self.parse_term()?;

        while let Some(op) = self.match_comparison_op() {
            let right = self.parse_term()?;
            expr = VirExpr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn match_comparison_op(&mut self) -> Option<VirOp> {
        if self.match_token(&[Token::Less]) {
            Some(VirOp::LessThan)
        } else if self.match_token(&[Token::LessEqual]) {
            Some(VirOp::LessEqual)
        } else if self.match_token(&[Token::Greater]) {
            Some(VirOp::GreaterThan)
        } else if self.match_token(&[Token::GreaterEqual]) {
            Some(VirOp::GreaterEqual)
        } else {
            None
        }
    }

    fn parse_term(&mut self) -> Result<VirExpr, String> {
        let mut expr = self.parse_factor()?;

        while let Some(op) = self.match_term_op() {
            let right = self.parse_factor()?;
            expr = VirExpr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn match_term_op(&mut self) -> Option<VirOp> {
        if self.match_token(&[Token::Plus]) {
            Some(VirOp::Add)
        } else if self.match_token(&[Token::Minus]) {
            Some(VirOp::Sub)
        } else {
            None
        }
    }

    fn parse_factor(&mut self) -> Result<VirExpr, String> {
        let mut expr = self.parse_primary()?;

        while let Some(op) = self.match_factor_op() {
            let right = self.parse_primary()?;
            expr = VirExpr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn match_factor_op(&mut self) -> Option<VirOp> {
        if self.match_token(&[Token::Star]) {
            Some(VirOp::Mul)
        } else if self.match_token(&[Token::Slash]) {
            Some(VirOp::Div)
        } else {
            None
        }
    }

    fn parse_primary(&mut self) -> Result<VirExpr, String> {
        if let Some(token) = self.advance() {
            match token {
                Token::Int(n) => Ok(VirExpr::Int(*n)),
                Token::Float(f) => Ok(VirExpr::Float(*f)),
                Token::String(s) => Ok(VirExpr::String(s.clone())),
                Token::Verdadeiro => Ok(VirExpr::Bool(true)),
                Token::Falso => Ok(VirExpr::Bool(false)),
                Token::Identifier(name) => {
                    let name = name.clone();
                    if self.match_token(&[Token::LParen]) {
                        let mut args = Vec::new();
                        if !self.check(&Token::RParen) {
                            loop {
                                args.push(self.parse_expression()?);
                                if self.match_token(&[Token::Comma]) {
                                    continue;
                                }
                                break;
                            }
                        }
                        if !self.match_token(&[Token::RParen]) {
                            return Err("Esperado ')' após argumentos da função.".to_string());
                        }
                        Ok(VirExpr::Call { name, args })
                    } else {
                        Ok(VirExpr::Var(name))
                    }
                }
                Token::LParen => {
                    let expr = self.parse_expression()?;
                    if !self.match_token(&[Token::RParen]) {
                        return Err("Esperado ')' após expressão.".to_string());
                    }
                    Ok(expr)
                }
                _ => Err(format!("Sintaxe inválida ao analisar expressão: {:?}", token)),
            }
        } else {
            Err("Fim inesperado do ficheiro durante a análise.".to_string())
        }
    }

    fn is_at_end(&self) -> bool {
        self.check(&Token::EOF) || self.current >= self.tokens.len()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    }

    fn check(&self, token: &Token) -> bool {
        if let Some(t) = self.peek() {
            t == token
        } else {
            false
        }
    }

    fn match_token(&mut self, types: &[Token]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            let token = self.tokens.get(self.current);
            self.current += 1;
            token
        } else {
            None
        }
    }
}
