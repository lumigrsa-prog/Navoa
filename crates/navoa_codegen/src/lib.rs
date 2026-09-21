// Exemplo de como devem estar estruturados os matches no teu codegen para evitar os avisos:
match statement {
    Statement::If { condition: _condition, then_branch: _then_branch, else_branch: _else_branch } => {
        // Lógica para o If
    }
    Statement::While { condition: _condition, body: _body } => {
        // Lógica para o While
    }
    // ... restantes ramos
}
