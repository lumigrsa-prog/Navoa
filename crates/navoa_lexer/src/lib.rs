#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Palavras-chave
    Se,
    Senao,
    Enquanto,
    Para,
    Funcao,
    Retornar,
    Imprimir,
    Var,
    Verdadeiro,
    Falso,
    Nulo,

    // Identificadores e Literais
    Identificador(String),
    Numero(f64),
    Texto(String),

    // Operadores e Pontuação
    Atribuicao,     // =
    Igual,          // ==
    Diferente,      // !=
    Maior,          // >
    Menor,          // <
    MaiorIgual,     // >=
    MenorIgual,     // <=
    Mais,           // +
    Menos,          // -
    Multiplicacao,  // *
    Divisao,        // /
    E,              // e, and, et
    Ou,             // ou, or, ou
    Nao,            // nao, not, non
    AbreParenteses, // (
    FechaParenteses,// )
    AbreChave,      // {
    FechaChave,     // }
    Virgula,        // ,
    EOF,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn novo(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    pub fn proximo_token(&mut self) -> Token {
        while let Some(ch) = self.peek() {
            // Ignorar espaços e ponto e vírgula
            if ch.is_whitespace() || ch == ';' {
                self.advance();
                continue;
            }

            // Comentários de linha
            if ch == '/' {
                let mut temp = self.input[self.pos..].chars();
                temp.next();
                if temp.next() == Some('/') {
                    while let Some(c) = self.peek() {
                        if c == '\n' { break; }
                        self.advance();
                    }
                    continue;
                }
            }

            // Strings
            if ch == '"' || ch == '\'' {
                let quote = self.advance().unwrap();
                let mut texto = String::new();
                while let Some(c) = self.peek() {
                    if c == quote {
                        self.advance();
                        break;
                    }
                    texto.push(c);
                    self.advance();
                }
                return Token::Texto(texto);
            }

            // Números
            if ch.is_ascii_digit() {
                let mut num_str = String::new();
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        num_str.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }
                let val = num_str.parse::<f64>().unwrap_or(0.0);
                return Token::Numero(val);
            }

            // Identificadores e Palavras-Chave Multi-Idioma
            if ch.is_alphabetic() || ch == '_' {
                let mut ident = String::new();
                while let Some(c) = self.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }

                return match ident.as_str() {
                    "se" | "if" | "si" => Token::Se,
                    "senao" | "else" | "sinon" => Token::Senao,
                    "enquanto" | "while" | "tantque" | "mientras" => Token::Enquanto,
                    "para" | "for" | "pour" => Token::Para,
                    "funcao" | "fn" | "function" | "fonction" => Token::Funcao,
                    "retornar" | "retorna" | "return" | "retourner" => Token::Retornar,
                    "imprimir" | "print" | "ecrire" | "escribir" => Token::Imprimir,
                    "var" | "let" => Token::Var,
                    "verdadeiro" | "true" | "vrai" => Token::Verdadeiro,
                    "falso" | "false" | "faux" => Token::Falso,
                    "nulo" | "null" | "nil" => Token::Nulo,
                    "e" | "and" | "et" => Token::E,
                    "ou" | "or" => Token::Ou,
                    "nao" | "not" | "non" => Token::Nao,
                    _ => Token::Identificador(ident),
                };
            }

            // Pontuação e Operadores
            self.advance();
            return match ch {
                '=' => {
                    if self.peek() == Some('=') { self.advance(); Token::Igual }
                    else { Token::Atribuicao }
                }
                '!' => {
                    if self.peek() == Some('=') { self.advance(); Token::Diferente }
                    else { Token::Nao }
                }
                '>' => {
                    if self.peek() == Some('=') { self.advance(); Token::MaiorIgual }
                    else { Token::Maior }
                }
                '<' => {
                    if self.peek() == Some('=') { self.advance(); Token::MenorIgual }
                    else { Token::Menor }
                }
                '+' => Token::Mais,
                '-' => Token::Menos,
                '*' => Token::Multiplicacao,
                '/' => Token::Divisao,
                '(' => Token::AbreParenteses,
                ')' => Token::FechaParenteses,
                '{' => Token::AbreChave,
                '}' => Token::FechaChave,
                ',' => Token::Virgula,
                _ => Token::EOF,
            };
        }
        Token::EOF
    }
}
