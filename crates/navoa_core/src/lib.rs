pub mod ast;
pub mod lexer;
pub mod parser;

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn eval(&mut self, code: &str) -> Result<String, String> {
        let code_trimmed = code.trim();
        if code_trimmed.is_empty() {
            return Ok("Código vazio.".to_string());
        }

        Ok(format!("Resultado: {code_trimmed}"))
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
