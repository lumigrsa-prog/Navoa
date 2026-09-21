use navoa_parser::Parser;
use navoa_analyzer::Analyzer;
use navoa_codegen::CodeGenerator;
use navoa_vm::VM;

pub fn execute_source(source: &str) -> Result<(), String> {
    // 1. Parsing (Lexer interno + AST em VIR)
    let mut parser = Parser::new(source);
    let stmts = parser.parse_program()?;

    // 2. Análise Semântica
    let mut analyzer = Analyzer::new();
    let analyzed_stmts = analyzer.analyze(stmts)?;

    // 3. Geração de Bytecode
    let mut codegen = CodeGenerator::new();
    let bytecode = codegen.generate(&analyzed_stmts)?;

    // 4. Execução na VM
    let mut vm = VM::new();
    vm.run(&bytecode)?;

    Ok(())
}
