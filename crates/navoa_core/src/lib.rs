pub type Interpreter = NavoaEngine;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Language { PT, EN, ES, FR, IT, DE }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub language: Language,
    pub chapter: u32,
    pub challenge_completed: bool,
    #[serde(default)]
    pub variables: HashMap<String, f64>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: Language::PT,
            chapter: 1,
            challenge_completed: false,
            variables: HashMap::new(),
        }
    }
}

pub struct NavoaEngine {
    pub config: Config,
}

impl NavoaEngine {
    pub fn new() -> Self {
        let config = Self::load_config().unwrap_or_default();
        Self { config }
    }

    fn config_path() -> PathBuf {
        let mut path = std::env::current_dir().unwrap_or_default();
        path.push(".navoa_config.json");
        path
    }

    pub fn load_config() -> Option<Config> {
        let path = Self::config_path();
        if path.exists() {
            let data = fs::read_to_string(path).ok()?;
            serde_json::from_str(&data).ok()
        } else {
            None
        }
    }

    pub fn save_config(&self) {
        let path = Self::config_path();
        if let Ok(data) = serde_json::to_string_pretty(&self.config) {
            let _ = fs::write(path, data);
        }
    }

    fn get_val(&self, s: &str) -> f64 {
        if let Ok(v) = s.parse::<f64>() {
            v
        } else {
            *self.config.variables.get(s).unwrap_or(&0.0)
        }
    }

    pub fn execute(&mut self, code: &str) -> String {
        let code = code.trim().to_lowercase();
        if code.is_empty() || code.starts_with("//") {
            return String::new();
        }

        // 1. SISTEMA DE PISTAS
        if code.starts_with("inspecionar ") {
            let obj = code.strip_prefix("inspecionar ").unwrap().trim();
            return match (self.config.chapter, obj) {
                (1, "quarto") => "🔍 PISTA: O quadro elétrico precisa de energia. Cria a variável 'energia' e dá-lhe o valor 100.".to_string(),
                (2, "maquina") => "🔍 PISTA: A máquina só liga se houver energia. Usa: se energia == 100 { cafe = 1 }".to_string(),
                (3, "cofre") => "🔍 PISTA: A fechadura está perra. Tens de somar 150 ao código 3 vezes. Usa: repetir 3 { codigo = codigo + 150 }".to_string(),
                (4, "sistema") => "🔍 PISTA: Se o código for 450, o risco é zero. Usa: se codigo == 450 { risco = 0 }".to_string(),
                (5, "porta") => "🔍 PISTA: A porta abre quando não há risco. Usa: se risco == 0 { saida = 1 }".to_string(),
                _ => format!("🔍 Inspecionaste '{}', mas não encontraste nada de útil.", obj),
            };
        }

        // 2. CICLOS DE REPETIÇÃO
        if code.starts_with("repetir ") && code.contains('{') && code.contains('}') {
            let parts: Vec<&str> = code.split('{').collect();
            let n_str = parts[0].replace("repetir", "").trim().to_string();
            let action = parts[1].replace("}", "").trim().to_string();

            if let Ok(n) = n_str.parse::<usize>() {
                let mut last_output = String::new();
                for _ in 0..n {
                    last_output = self.parse_assignment(&action);
                }
                return format!("🔄 Ciclo executado {} vezes!\n{}", n, last_output);
            }
        }

        // 3. CONDIÇÕES LÓGICAS
        if code.starts_with("se ") && code.contains("==") && code.contains('{') && code.contains('}') {
            let condition_parts: Vec<&str> = code.split('{').collect();
            let condition = condition_parts[0].replace("se", "").trim().to_string();
            let action = condition_parts[1].replace("}", "").trim().to_string();

            let vars: Vec<&str> = condition.split("==").collect();
            if vars.len() == 2 {
                let var_val = self.get_val(vars[0].trim());
                let test_val = self.get_val(vars[1].trim());

                if var_val == test_val {
                    let res = self.parse_assignment(&action);
                    return format!("✅ Condição verdadeira ({} == {}).\n{}", var_val, test_val, res);
                } else {
                    return format!("❌ Condição falsa ({} não é igual a {}). Ação ignorada.", var_val, test_val);
                }
            }
        }

        // 4. ATRIBUIÇÕES SIMPLES OU SOMAS
        if code.contains('=') {
            return self.parse_assignment(&code);
        }

        "❓ Comando não reconhecido. Tenta usar 'inspecionar <algo>'.".to_string()
    }

    fn parse_assignment(&mut self, code: &str) -> String {
        let parts: Vec<&str> = code.split('=').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            let var_name = parts[0].to_string();
            let expr = parts[1];

            let val = if expr.contains('+') {
                let sub: Vec<&str> = expr.split('+').collect();
                self.get_val(sub[0].trim()) + self.get_val(sub[1].trim())
            } else {
                self.get_val(expr)
            };

            self.config.variables.insert(var_name.clone(), val);
            self.save_config();
            self.check_progression(&var_name, val)
        } else {
            "⚠️ Erro de sintaxe. Usa: variavel = valor".to_string()
        }
    }

    fn check_progression(&mut self, var: &str, val: f64) -> String {
        let mut output = format!("📦 Variável '{}' definida como {}!\n", var, val);

        if self.config.chapter == 1 && var == "energia" && val == 100.0 {
            self.config.chapter = 2;
            output.push_str("💡 A energia flui. Os sistemas do quarto ligam-se.");
        } else if self.config.chapter == 2 && var == "cafe" && val == 1.0 {
            self.config.chapter = 3;
            output.push_str("☕ Bebes o café. O cérebro desperta para o próximo passo.");
        } else if self.config.chapter == 3 && var == "codigo" && val == 450.0 {
            self.config.chapter = 4;
            output.push_str("🔓 O cofre abre-se com um clique pesado.");
        } else if self.config.chapter == 4 && var == "risco" && val == 0.0 {
            self.config.chapter = 5;
            output.push_str("📊 Risco neutralizado. A porta de segurança destranca.");
        } else if self.config.chapter == 5 && var == "saida" && val == 1.0 {
            self.config.challenge_completed = true;
            output.push_str("🚪 Estás fora. As ruas aguardam-te.");
        }
        self.save_config();
        output
    }
}
