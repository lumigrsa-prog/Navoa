#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_array_literal() {
        let input = "[1, 2, 3]";

        // 1. Tokenização do input
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        // 2. Parsing para a estrutura AST
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Falha ao analisar a sintaxe da lista");

        // 3. Validação da estrutura AST gerada
        match ast {
            Expr::Array(elements) => {
                assert_eq!(elements.len(), 3, "A lista deve conter exatamente 3 elementos");
                assert_eq!(elements[0], Expr::Number(1.0));
                assert_eq!(elements[1], Expr::Number(2.0));
                assert_eq!(elements[2], Expr::Number(3.0));
            }
            _ => panic!("Esperado Expr::Array, mas foi retornado: {:?}", ast),
        }
    }

    #[test]
    fn test_parse_empty_array() {
        let input = "[]";
        let tokens = Lexer::new(input).tokenize();
        let ast = Parser::new(tokens).parse().expect("Falha no parser para lista vazia");

        if let Expr::Array(elements) = ast {
            assert!(elements.is_empty(), "A lista vazia deve conter 0 elementos");
        } else {
            panic!("Esperado Expr::Array para sintaxe '[]'");
        }
    }
}
