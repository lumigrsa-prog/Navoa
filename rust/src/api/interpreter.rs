use flutter_rust_bridge::frb;
use navoa_core::lexer::Lexer;
use navoa_core::parser::Parser;
use navoa_core::vm::VM;

pub struct NavoaSession;

impl NavoaSession {
    #[frb(sync)]
    pub fn new() -> Self {
        Self
    }

    pub fn execute_code(&self, code: String) -> Result<String, String> {
        let mut lexer = Lexer::new(&code);
        let tokens = lexer.tokenize();

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().map_err(|e| format!("Erro Sintático: {}", e))?;

        let mut vm = VM::new();
        let result = vm.interpret(&ast).map_err(|e| format!("Erro de Execução: {}", e))?;

        Ok(format!("{:?}", result))
    }
}
