use flutter_rust_bridge::frb;
use navoa_core::NavoaEngine;

pub struct InterpreterBridge {
    engine: NavoaEngine,
}

impl InterpreterBridge {
    #[frb(sync)]
    pub fn new() -> Self {
        Self {
            engine: NavoaEngine::new(),
        }
    }

    #[frb(sync)]
    pub fn execute(&mut self, code: String) -> String {
        self.engine.execute(&code)
    }
}
