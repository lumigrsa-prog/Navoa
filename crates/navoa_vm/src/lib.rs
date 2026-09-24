use navoa_ast::{Expressao, Instrucao, Programa};
use std::collections::HashMap;

pub struct VM {
    variaveis: HashMap<String, f64>,
    saida: Vec<String>,
}

impl VM {
    pub fn nova() -> Self {
        Self {
            variaveis: HashMap::new(),
            saida: Vec::new(),
        }
    }

    pub fn executar(&mut self, programa: &Programa) {
        for inst in &programa.instrucoes {
            self.executar_instrucao(inst);
        }
    }

    fn executar_instrucao(&mut self, inst: &Instrucao) {
        match inst {
            Instrucao::Imprimir(expr) => {
                let val = self.avaliar_expressao(expr);
                self.saida.push(val);
            }
            Instrucao::Atribuicao { nome, valor } => {
                if let Expressao::Numero(n) = valor {
                    self.variaveis.insert(nome.clone(), *n);
                }
            }
            Instrucao::Se { condicao, bloco_entao, bloco_senao } => {
                if self.e_verdadeiro(condicao) {
                    for sub in bloco_entao {
                        self.executar_instrucao(sub);
                    }
                } else if let Some(senao) = bloco_senao {
                    for sub in senao {
                        self.executar_instrucao(sub);
                    }
                }
            }
            Instrucao::Enquanto { condicao, bloco } => {
                while self.e_verdadeiro(condicao) {
                    for sub in bloco {
                        self.executar_instrucao(sub);
                    }
                }
            }
        }
    }

    fn e_verdadeiro(&self, expr: &Expressao) -> bool {
        match expr {
            Expressao::Booleano(b) => *b,
            Expressao::Numero(n) => *n != 0.0,
            _ => false,
        }
    }

    fn avaliar_expressao(&self, expr: &Expressao) -> String {
        match expr {
            Expressao::Numero(n) => n.to_string(),
            Expressao::Texto(t) => t.clone(),
            Expressao::Variavel(nome) => {
                self.variaveis.get(nome).map(|v| v.to_string()).unwrap_or_else(|| "0".to_string())
            }
            Expressao::Booleano(b) => b.to_string(),
        }
    }

    pub fn obter_saida(&self) -> String {
        self.saida.join("\n")
    }
}
