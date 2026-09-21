// Importa as estruturas do parser (ajusta o caminho se necessário, ex: navoa_parser)
use navoa_parser::{Statement, Expr, BinaryOp};
pub struct CodeGen {
    // Podes adicionar aqui os campos para bytecode ou instruções geradas
}

impl CodeGen {
    /// Cria uma nova instância do gerador de código
    pub fn new() -> Self {
        Self {}
    }

    /// O método chamado pelo CLI, adaptado para aceitar a lista de Statement
    pub fn compile(&mut self, statements: &[Statement]) -> Result<(), String> {
        for statement in statements {
            self.compile_statement(statement)?;
        }
        
        Ok(())
    }

    /// Processa cada Statement individualmente com base na tua AST
    fn compile_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::VarDecl { name, value } => {
                // TODO: Compilar a declaração de variável
            }
            Statement::Assign { name, value } => {
                // TODO: Compilar a atribuição de variável
            }
            Statement::FunctionDecl { name, params, body } => {
                // TODO: Compilar a declaração de função
            }
            Statement::Return(expr) => {
                // TODO: Compilar a instrução return
            }
            Statement::Print(expr) => {
                // TODO: Compilar a instrução print
            }
            Statement::Expr(expr) => {
                // TODO: Compilar uma expressão isolada
            }
            Statement::If { condition, then_branch, else_branch } => {
                // TODO: Compilar a estrutura condicional if/else
            }
            Statement::While { condition, body } => {
                // TODO: Compilar o ciclo while
            }
        }
        
        Ok(())
    }
}
