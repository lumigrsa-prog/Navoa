#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    Portuguese,
    English,
    Spanish,
    French,
    Italian,
    German,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Var,
    Print,
    While,
    Identifier(String),
    Number(f64),
    String(String),
    Equal,
    EqualEqual,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Eof,
}

pub struct Lexer<'a> {
    chars: std::str::Chars<'a>,
    current_char: Option<char>,
    lang: Language,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, lang: Language) -> Self {
        let mut chars = source.chars();
        let current_char = chars.next();
        Lexer {
            chars,
            current_char,
            lang,
        }
    }

    fn advance(&mut self) {
        self.current_char = self.chars.next();
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let c = match self.current_char {
            Some(ch) => ch,
            None => return Token::Eof,
        };

        if c.is_alphabetic() || c == '_' {
            return self.identifier_or_keyword();
        }

        if c.is_ascii_digit() {
            return self.number();
        }

        if c == '"' {
            return self.string_literal();
        }

        match c {
            '=' => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::EqualEqual
                } else {
                    Token::Equal
                }
            }
            '!' => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::BangEqual
                } else {
                    Token::BangEqual
                }
            }
            '>' => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            '<' => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            '+' => { self.advance(); Token::Plus }
            '-' => { self.advance(); Token::Minus }
            '*' => { self.advance(); Token::Star }
            '/' => { self.advance(); Token::Slash }
            '(' => { self.advance(); Token::LParen }
            ')' => { self.advance(); Token::RParen }
            '{' => { self.advance(); Token::LBrace }
            '}' => { self.advance(); Token::RBrace }
            ';' => { self.advance(); Token::Semicolon }
            _ => {
                self.advance();
                self.next_token()
            }
        }
    }

    fn identifier_or_keyword(&mut self) -> Token {
        let mut s = String::new();
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let lower = s.to_lowercase();
        match self.lang {
            Language::Portuguese => match lower.as_str() {
                "var" => Token::Var,
                "imprimir" => Token::Print,
                "enquanto" => Token::While,
                _ => Token::Identifier(s),
            },
            Language::English => match lower.as_str() {
                "var" => Token::Var,
                "print" => Token::Print,
                "while" => Token::While,
                _ => Token::Identifier(s),
            },
            Language::Spanish => match lower.as_str() {
                "var" => Token::Var,
                "imprimir" => Token::Print,
                "mientras" => Token::While,
                _ => Token::Identifier(s),
            },
            Language::French => match lower.as_str() {
                "var" => Token::Var,
                "imprimer" => Token::Print,
                "tantque" => Token::While,
                _ => Token::Identifier(s),
            },
            Language::Italian => match lower.as_str() {
                "var" => Token::Var,
                "stampa" => Token::Print,
                "finche" => Token::While,
                _ => Token::Identifier(s),
            },
            Language::German => match lower.as_str() {
                "var" => Token::Var,
                "drucken" => Token::Print,
                "solange" => Token::While,
                _ => Token::Identifier(s),
            },
        }
    }

    fn number(&mut self) -> Token {
        let mut s = String::new();
        let mut decimal = false;
        while let Some(c) = self.current_char {
            if c.is_ascii_digit() {
                s.push(c);
                self.advance();
            } else if c == '.' && !decimal {
                decimal = true;
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token::Number(s.parse().unwrap_or(0.0))
    }

    fn string_literal(&mut self) -> Token {
        self.advance();
        let mut s = String::new();
        while let Some(c) = self.current_char {
            if c == '"' {
                self.advance();
                break;
            } else {
                s.push(c);
                self.advance();
            }
        }
        Token::String(s)
    }
}
