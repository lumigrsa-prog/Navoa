use navoa_ast::{Expressao, Instrucao, Programa};
use std::collections::HashMap;

#[derive(Clone)]
struct DefFuncao {
    parametros: Vec<String>,
    corpo: Vec<Instrucao>,
}

pub struct VM {
    funcoes: HashMap<String, DefFuncao>,
    escopos: Vec<HashMap<String, String>>,
    saida: Vec<String>,
    valor_retorno: Option<String>,
}

impl VM {
    pub fn nova() -> Self {
        let mut vm = Self {
            funcoes: HashMap::new(),
            escopos: Vec::new(),
            saida: Vec::new(),
            valor_retorno: None,
        };
        vm.escopos.push(HashMap::new());
        vm
    }

    pub fn executar(&mut self, programa: &Programa) {
        for inst in &programa.instrucoes {
            if self.valor_retorno.is_some() {
                break;
            }
            self.executar_instrucao(inst);
        }
    }

    fn executar_instrucao(&mut self, inst: &Instrucao) {
        if self.valor_retorno.is_some() {
            return;
        }

        match inst {
            Instrucao::Imprimir(expr) => {
                let val = self.avaliar_expressao(expr);
                self.saida.push(val);
            }
            Instrucao::Atribuicao { nome, valor } => {
                let val = self.avaliar_expressao(valor);
                if let Some(escopo) = self.escopos.last_mut() {
                    escopo.insert(nome.clone(), val);
                }
            }
            Instrucao::DeclararFuncao { nome, parametros, corpo } => {
                self.funcoes.insert(
                    nome.clone(),
                    DefFuncao {
                        parametros: parametros.clone(),
                        corpo: corpo.clone(),
                    },
                );
            }
            Instrucao::Retornar(opt_expr) => {
                let val = match opt_expr {
                    Some(expr) => self.avaliar_expressao(expr),
                    None => "nulo".to_string(),
                };
                self.valor_retorno = Some(val);
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
                while self.e_verdadeiro(condicao) && self.valor_retorno.is_none() {
                    for sub in bloco {
                        self.executar_instrucao(sub);
                    }
                }
            }
            Instrucao::Expressao(expr) => {
                self.avaliar_expressao(expr);
            }
        }
    }

    fn e_verdadeiro(&mut self, expr: &Expressao) -> bool {
        let val = self.avaliar_expressao(expr);
        val == "true" || val == "verdadeiro" || (val.parse::<f64>().map(|n| n != 0.0).unwrap_or(false))
    }

    fn avaliar_expressao(&mut self, expr: &Expressao) -> String {
        match expr {
            Expressao::Numero(n) => n.to_string(),
            Expressao::Texto(t) => t.clone(),
            Expressao::Booleano(b) => b.to_string(),
            Expressao::Variavel(nome) => {
                for escopo in self.escopos.iter().rev() {
                    if let Some(val) = escopo.get(nome) {
                        return val.clone();
                    }
                }
                "0".to_string()
            }
            Expressao::Chamada { nome, argumentos } => {
                let args_avaliados: Vec<String> = argumentos
                    .iter()
                    .map(|arg| self.avaliar_expressao(arg))
                    .collect();

                if let Some(def) = self.funcoes.get(nome).cloned() {
                    let mut novo_escopo = HashMap::new();
                    for (param, val) in def.parametros.iter().zip(args_avaliados.iter()) {
                        novo_escopo.insert(param.clone(), val.clone());
                    }

                    self.escopos.push(novo_escopo);
                    let ret_anterior = self.valor_retorno.take();

                    for sub in &def.corpo {
                        if self.valor_retorno.is_some() {
                            break;
                        }
                        self.executar_instrucao(sub);
                    }

                    let resultado = self.valor_retorno.take().unwrap_or_else(|| "nulo".to_string());
                    self.valor_retorno = ret_anterior;
                    self.escopos.pop();

                    resultado
                } else {
                    format!("Erro: Função '{}' não definida", nome)
                }
            }
        }
    }

    pub fn obter_saida(&self) -> String {
        self.saida.join("\n")
    }
}
