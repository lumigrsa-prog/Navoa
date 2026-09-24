#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Palavras-chave e Estruturas
    Imprimir,
    Var,
    Se,
    Senao,
    Enquanto,
    Funcao,
    Retornar,
    
    // Valores
    Identificador(String),
    Numero(f64),
    Texto(String),
    Verdadeiro,
    Falso,

    // Simbolos e Pontuação
    Atribuicao,
    AbreParenteses,
    FechaParenteses,
    AbreChave,
    FechaChave,
    Virgula,

    // Operadores Aritmeticos e Logicos
    Mais,
    Menos,
    Asterisco,
    Barra,
    IgualIgual,
    Diferente,
    Menor,
    Maior,
    MenorIgual,
    MaiorIgual,

    EOF,
}

pub struct Lexer<'a> {
    input: &'a str,
    posicao: usize,
}

impl<'a> Lexer<'a> {
    pub fn novo(input: &'a str) -> Self {
        Self { input, posicao: 0 }
    }

    fn char_atual(&self) -> Option<char> {
        self.input[self.posicao..].chars().next()
    }

    fn espiar_proximo(&self) -> Option<char> {
        let mut chars = self.input[self.posicao..].chars();
        chars.next();
        chars.next()
    }

    fn avançar(&mut self) {
        if let Some(c) = self.char_atual() {
            self.posicao += c.len_utf8();
        }
    }

    pub fn proximo_token(&mut self) -> Token {
        while let Some(c) = self.char_atual() {
            if c.is_whitespace() {
                self.avançar();
                continue;
            }

            // Comentários de linha única //
            if c == '/' && self.espiar_proximo() == Some('/') {
                while let Some(ch) = self.char_atual() {
                    if ch == '\n' { break; }
                    self.avançar();
                }
                continue;
            }

            // Operadores compostos e simples
            match c {
                '+' => { self.avançar(); return Token::Mais; }
                '-' => { self.avançar(); return Token::Menos; }
                '*' => { self.avançar(); return Token::Asterisco; }
                '/' => { self.avançar(); return Token::Barra; }
                '=' => {
                    self.avançar();
                    if self.char_atual() == Some('=') {
                        self.avançar();
                        return Token::IgualIgual;
                    }
                    return Token::Atribuicao;
                }
                '!' => {
                    if self.espiar_proximo() == Some('=') {
                        self.avançar();
                        self.avançar();
                        return Token::Diferente;
                    }
                }
                '<' => {
                    self.avançar();
                    if self.char_atual() == Some('=') {
                        self.avançar();
                        return Token::MenorIgual;
                    }
                    return Token::Menor;
                }
                '>' => {
                    self.avançar();
                    if self.char_atual() == Some('=') {
                        self.avançar();
                        return Token::MaiorIgual;
                    }
                    return Token::Maior;
                }
                '(' => { self.avançar(); return Token::AbreParenteses; }
                ')' => { self.avançar(); return Token::FechaParenteses; }
                '{' => { self.avançar(); return Token::AbreChave; }
                '}' => { self.avançar(); return Token::FechaChave; }
                ',' => { self.avançar(); return Token::Virgula; }
                '"' => {
                    self.avançar();
                    let mut texto = String::new();
                    while let Some(ch) = self.char_atual() {
                        if ch == '"' {
                            self.avançar();
                            break;
                        }
                        texto.push(ch);
                        self.avançar();
                    }
                    return Token::Texto(texto);
                }
                _ => {}
            }

            if c.is_ascii_digit() {
                let mut num_str = String::new();
                while let Some(ch) = self.char_atual() {
                    if ch.is_ascii_digit() || ch == '.' {
                        num_str.push(ch);
                        self.avançar();
                    } else {
                        break;
                    }
                }
                if let Ok(num) = num_str.parse::<f64>() {
                    return Token::Numero(num);
                }
            }

            if c.is_alphabetic() || c == '_' {
                let mut ident = String::new();
                while let Some(ch) = self.char_atual() {
                    if ch.is_alphanumeric() || ch == '_' {
                        ident.push(ch);
                        self.avançar();
                    } else {
                        break;
                    }
                }

                return match ident.as_str() {
                    "imprimir" | "print" | "ecrire" | "escribir" | "stampare" | "drucken" => Token::Imprimir,
                    "var" | "let" => Token::Var,
                    "se" | "if" | "si" | "wenn" => Token::Se,
                    "senao" | "else" | "sinon" | "sino" | "altrimenti" | "sonst" => Token::Senao,
                    "enquanto" | "while" | "tantque" | "mientras" | "mentre" | "solange" => Token::Enquanto,
                    "funcao" | "fn" | "function" | "fonction" | "funcion" | "funzione" | "funktion" => Token::Funcao,
                    "retornar" | "return" | "retourner" | "ritornare" | "rueckgabe" => Token::Retornar,
                    "verdadeiro" | "true" | "vrai" | "verdadero" | "vero" | "wahr" => Token::Verdadeiro,
                    "falso" | "false" | "faux" | "falso_" | "falsch" => Token::Falso,
                    _ => Token::Identificador(ident),
                };
            }

            self.avançar();
        }

        Token::EOF
    }
}
