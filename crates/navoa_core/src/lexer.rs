#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Identifier(String),
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            let ch = self.input[self.position];

            match ch {
                ' ' | '\t' | '\r' | '\n' => {
                    self.position += 1;
                }
                '[' => {
                    tokens.push(Token::LeftBracket);
                    self.position += 1;
                }
                ']' => {
                    tokens.push(Token::RightBracket);
                    self.position += 1;
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.position += 1;
                }
                c if c.is_ascii_digit() => {
                    let mut num_str = String::new();
                    while self.position < self.input.len()
                        && (self.input[self.position].is_ascii_digit() || self.input[self.position] == '.')
                    {
                        num_str.push(self.input[self.position]);
                        self.position += 1;
                    }
                    if let Ok(n) = num_str.parse::<f64>() {
                        tokens.push(Token::Number(n));
                    }
                }
                c if c.is_alphabetic() || c == '_' => {
                    let mut id_str = String::new();
                    while self.position < self.input.len()
                        && (self.input[self.position].is_alphanumeric() || self.input[self.position] == '_')
                    {
                        id_str.push(self.input[self.position]);
                        self.position += 1;
                    }
                    tokens.push(Token::Identifier(id_str));
                }
                _ => {
                    self.position += 1;
                }
            }
        }

        tokens.push(Token::EOF);
        tokens
    }
}
