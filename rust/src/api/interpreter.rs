use navoa_core::Interpreter;

pub struct NavoaSession {}

impl NavoaSession {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute_code(&self, code: String) -> String {
        let mut interpreter = Interpreter::new();
        match interpreter.eval(&code) {
            Ok(value) => value,
            Err(err) => format!("Erro: {err}"),
        }
    }
}
