use navoa_ast::{Expr, Statement};
use navoa_lexer::Token;

pub struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, position: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.position);
        if tok.is_some() {
            self.position += 1;
        }
        tok
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        while let Some(tok) = self.peek() {
            if tok == &Token::EOF {
                break;
            }
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.peek() {
            Some(Token::Imprimir) => {
                self.advance();
                let expr = self.parse_expression()?;
                Ok(Statement::Imprimir(expr))
            }
            Some(Token::Var) => {
                self.advance();
                if let Some(Token::Identificador(nome)) = self.advance().cloned() {
                    if let Some(Token::Atribuicao) = self.advance() {
                        let expr = self.parse_expression()?;
                        Ok(Statement::Atribuir(nome, expr))
                    } else {
                        Err("Esperado '=' após o nome da variável.".to_string())
                    }
                } else {
                    Err("Esperado identificador após 'var'.".to_string())
                }
            }
            Some(Token::Identificador(nome)) => {
                let nome_var = nome.clone();
                if self.tokens.get(self.position + 1) == Some(&Token::Atribuicao) {
                    self.advance(); // consome o identificador
                    self.advance(); // consome '='
                    let expr = self.parse_expression()?;
                    Ok(Statement::Atribuir(nome_var, expr))
                } else {
                    Err(format!("Sintaxe não reconhecida próxima a '{}'", nome_var))
                }
            }
            Some(t) => Err(format!("Token inesperado no início da instrução: {:?}", t)),
            None => Err("Fim de ficheiro inesperado.".to_string()),
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        let em_parenteses = if self.peek() == Some(&Token::AbreParenteses) {
            self.advance();
            true
        } else {
            false
        };

        let expr = match self.advance() {
            Some(Token::Numero(n)) => Expr::Numero(*n),
            Some(Token::Texto(s)) => Expr::Texto(s.clone()),
            Some(Token::Identificador(nome)) => Expr::Identificador(nome.clone()),
            Some(t) => return Err(format!("Expressão inválida próxima a {:?}", t)),
            None => return Err("Expressão incompleta.".to_string()),
        };

        if em_parenteses {
            if self.peek() == Some(&Token::FechaParenteses) {
                self.advance();
            } else {
                return Err("Esperado ')' após expressão.".to_string());
            }
        }

        Ok(expr)
    }
}
