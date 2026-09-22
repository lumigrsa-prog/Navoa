#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Pt,
    Es,
    Fr,
    It,
    De,
    En,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Imprimir,
    Variavel,
    Se,
    Senao,
    Ler,
    Identificador(String),
    Numero(f64),
    Texto(String),
    Igual,
    Mais,
    Menos,
    Multiplicar,
    Dividir,
    ParentesisEsquerdo,
    ParentesisDireito,
    FimInstrucao,
}

pub struct Lexer<'a> {
    _input: &'a str,
    _language: Language,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, language: Language) -> Self {
        Self {
            _input: input,
            _language: language,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let tokens = Vec::new();
        Ok(tokens)
    }
}
