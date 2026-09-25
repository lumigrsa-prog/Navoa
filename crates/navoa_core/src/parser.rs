use crate::ast::Expr;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        let mut expr = match self.current() {
            Token::Number(n) => {
                let val = *n;
                self.advance();
                Expr::Number(val)
            }
            Token::Identifier(name) => {
                let id = name.clone();
                self.advance();
                Expr::Identifier(id)
            }
            Token::LeftBracket => {
                self.advance(); // Consome '['
                let mut elements = Vec::new();

                if *self.current() != Token::RightBracket {
                    loop {
                        elements.push(self.parse_expression()?);

                        if *self.current() == Token::Comma {
                            self.advance(); // Consome ','
                        } else {
                            break;
                        }
                    }
                }

                if *self.current() == Token::RightBracket {
                    self.advance(); // Consome ']'
                    Expr::Array(elements)
                } else {
                    return Err("Esperado ']' no fecho da lista".to_string());
                }
            }
            tok => return Err(format!("Sintaxe inesperada: {:?}", tok)),
        };

        // Suporte a acesso por índice: ex. lista[0]
        if *self.current() == Token::LeftBracket {
            self.advance(); // Consome '['
            let index = self.parse_expression()?;
            if *self.current() == Token::RightBracket {
                self.advance(); // Consome ']'
                expr = Expr::IndexAccess {
                    target: Box::new(expr),
                    index: Box::new(index),
                };
            } else {
                return Err("Esperado ']' após a chave de índice".to_string());
            }
        }

        Ok(expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_array_literal() {
        let input = "[1, 2, 3]";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Falha ao analisar a sintaxe da lista");

        assert_eq!(
            ast,
            Expr::Array(vec![
                Expr::Number(1.0),
                Expr::Number(2.0),
                Expr::Number(3.0)
            ])
        );
    }

    #[test]
    fn test_parse_empty_array() {
        let input = "[]";
        let tokens = Lexer::new(input).tokenize();
        let ast = Parser::new(tokens).parse().expect("Falha no parser para lista vazia");

        assert_eq!(ast, Expr::Array(vec![]));
    }

    #[test]
    fn test_parse_index_access() {
        let input = "lista[0]";
        let tokens = Lexer::new(input).tokenize();
        let ast = Parser::new(tokens).parse().expect("Falha no parser de acesso por índice");

        assert_eq!(
            ast,
            Expr::IndexAccess {
                target: Box::new(Expr::Identifier("lista".to_string())),
                index: Box::new(Expr::Number(0.0)),
            }
        );
    }
}
