use navoa_core::Interpreter;

pub struct NavoaSession {
    interpreter: Interpreter,
}

impl NavoaSession {
    #[frb(sync)]
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
        }
    }

    pub fn execute_code(&mut self, code: String) -> String {
        self.interpreter.execute(&code)
    }
}
