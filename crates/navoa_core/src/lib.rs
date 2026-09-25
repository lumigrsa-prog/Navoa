use std::collections::HashMap;

pub struct Interpreter {
    variables: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn execute(&mut self, code: &str) -> String {
        let mut output = String::new();

        for line in code.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            // Atribuição com sinal = (ex: x = 20 + 5 ou 20 + 5 = x)
            if line.contains('=') {
                let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    let (var_name, expr) = if parts[0].chars().all(|c| c.is_alphabetic() || c == '_') {
                        (parts[0], parts[1])
                    } else if parts[1].chars().all(|c| c.is_alphabetic() || c == '_') {
                        (parts[1], parts[0])
                    } else {
                        output.push_str("Erro: Nome de variável inválido.\n");
                        continue;
                    };

                    match self.eval_expr(expr) {
                        Ok(val) => {
                            self.variables.insert(var_name.to_string(), val);
                        }
                        Err(e) => {
                            output.push_str(&format!("Erro de sintaxe: {}\n", e));
                        }
                    }
                }
            } 
            // Comando print / println (ex: print x ou println(x))
            else if line.starts_with("print ") || line.starts_with("println ") || line.starts_with("print(") || line.starts_with("println(") {
                let content = if line.contains('(') && line.ends_with(')') {
                    let start = line.find('(').unwrap() + 1;
                    &line[start..line.len() - 1]
                } else {
                    let first_space = line.find(' ').unwrap_or(0);
                    &line[first_space..]
                }.trim();

                // Verificar se é uma variável guardada
                if let Some(val) = self.variables.get(content) {
                    output.push_str(&format!("{}\n", val));
                } else if let Ok(val) = self.eval_expr(content) {
                    output.push_str(&format!("{}\n", val));
                } else {
                    // Impressão de texto simples (remover aspas se existirem)
                    let text = content.trim_matches('"').trim_matches('\'');
                    output.push_str(&format!("{}\n", text));
                }
            } else {
                // Tentar avaliar como expressão direta
                match self.eval_expr(line) {
                    Ok(val) => output.push_str(&format!("{}\n", val)),
                    Err(_) => output.push_str(&format!("Comando não reconhecido: {}\n", line)),
                }
            }
        }

        if output.is_empty() {
            "Executado com sucesso.".to_string()
        } else {
            output.trim_end().to_string()
        }
    }

    fn eval_expr(&self, expr: &str) -> Result<f64, String> {
        let expr = expr.trim();

        // Se for um número direto
        if let Ok(val) = expr.parse::<f64>() {
            return Ok(val);
        }

        // Se for uma variável existente
        if let Some(&val) = self.variables.get(expr) {
            return Ok(val);
        }

        // Adição (+)
        if expr.contains('+') {
            let parts: Vec<&str> = expr.split('+').collect();
            let mut sum = 0.0;
            for part in parts {
                sum += self.eval_expr(part)?;
            }
            return Ok(sum);
        }

        // Subtração (-)
        if expr.contains('-') {
            let parts: Vec<&str> = expr.split('-').collect();
            let mut result = self.eval_expr(parts[0])?;
            for part in &parts[1..] {
                result -= self.eval_expr(part)?;
            }
            return Ok(result);
        }

        // Multiplicação (*)
        if expr.contains('*') {
            let parts: Vec<&str> = expr.split('*').collect();
            let mut prod = 1.0;
            for part in parts {
                prod *= self.eval_expr(part)?;
            }
            return Ok(prod);
        }

        // Divisão (/)
        if expr.contains('/') {
            let parts: Vec<&str> = expr.split('/').collect();
            let mut result = self.eval_expr(parts[0])?;
            for part in &parts[1..] {
                let val = self.eval_expr(part)?;
                if val == 0.0 {
                    return Err("Divisão por zero".to_string());
                }
                result /= val;
            }
            return Ok(result);
        }

        Err(format!("Expressão inválida: {}", expr))
    }
}
