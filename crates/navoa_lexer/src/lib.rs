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
    E,              // e, and, et, y, und
    Ou,             // ou, or, o, oder
    Nao,            // nao, not, non, no, nicht
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

            // Identificadores e Palavras-Chave (PT, EN, FR, ES, IT, DE)
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
                    // Condicionais (PT: se | EN: if | FR/ES/IT: si, se | DE: wenn, falls)
                    "se" | "if" | "si" | "wenn" | "falls" => Token::Se,
                    // Senão (PT: senao | EN: else | FR: sinon | ES: sino | IT: altrimenti | DE: sonst)
                    "senao" | "else" | "sinon" | "sino" | "altrimenti" | "sonst" => Token::Senao,

                    // Repetição (PT: enquanto | EN: while | FR: tantque | ES: mientras | IT: mentre | DE: solange, waehrend, während)
                    "enquanto" | "while" | "tantque" | "mientras" | "mentre" | "solange" | "waehrend" | "während" => Token::Enquanto,
                    // Para (PT/ES: para | EN: for | FR: pour | IT: per | DE: fuer, für)
                    "para" | "for" | "pour" | "per" | "fuer" | "für" => Token::Para,

                    // Funções e Retorno (PT: funcao | EN: fn, function | FR: fonction | ES: funcion | IT: funzione | DE: funktion)
                    "funcao" | "fn" | "function" | "fonction" | "funcion" | "funzione" | "funktion" => Token::Funcao,
                    "retornar" | "retorna" | "return" | "retourner" | "ritornare" | "ritorna" | "rueckgabe" | "rückgabe" | "zurueck" => Token::Retornar,

                    // Saída (PT: imprimir | EN: print | FR: ecrire | ES: escribir | IT: stampare, scrivere | DE: drucken, ausgeben)
                    "imprimir" | "print" | "ecrire" | "escribir" | "stampare" | "scrivere" | "drucken" | "ausgeben" => Token::Imprimir,

                    // Variáveis
                    "var" | "let" => Token::Var,

                    // Valores lógicos (PT: verdadeiro | EN: true | FR: vrai | ES: verdadero | IT: vero | DE: wahr)
                    "verdadeiro" | "true" | "vrai" | "verdadero" | "vero" | "wahr" => Token::Verdadeiro,
                    // Falso (PT/ES/IT: falso | EN: false | FR: faux | DE: falsch)
                    "falso" | "false" | "faux" | "falsch" => Token::Falso,
                    // Nulo (PT/ES: nulo | EN: null, nil | FR: nul | IT: nullo | DE: null)
                    "nulo" | "null" | "nil" | "nul" | "nullo" => Token::Nulo,

                    // Operadores lógicos (PT: e | EN: and | FR: et | ES: y | DE: und)
                    "e" | "and" | "et" | "y" | "und" => Token::E,
                    // Ou (PT/FR: ou | EN: or | ES/IT: o | DE: oder)
                    "ou" | "or" | "o" | "oder" => Token::Ou,
                    // Não (PT: nao | EN: not | FR/IT: non | ES: no | DE: nicht)
                    "nao" | "not" | "non" | "no" | "nicht" => Token::Nao,

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords_multilingues() {
        // PT, EN, FR, ES, IT, DE
        let codigo = "imprimir print ecrire escribir stampare drucken";
        let mut lexer = Lexer::novo(codigo);
        for _ in 0..6 {
            assert_eq!(lexer.proximo_token(), Token::Imprimir);
        }

        let condicionais = "se if si wenn";
        let mut lexer = Lexer::novo(condicionais);
        for _ in 0..4 {
            assert_eq!(lexer.proximo_token(), Token::Se);
        }

        let lacos = "enquanto while tantque mientras mentre solange";
        let mut lexer = Lexer::novo(lacos);
        for _ in 0..6 {
            assert_eq!(lexer.proximo_token(), Token::Enquanto);
        }
    }
}
