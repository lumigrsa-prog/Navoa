#[derive(Debug, PartialEq, Clone)]
pub enum Expressao {
    Numero(f64),
    Texto(String),
    Variavel(String),
    Booleano(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instrucao {
    Imprimir(Expressao),
    Atribuicao {
        nome: String,
        valor: Expressao,
    },
    Se {
        condicao: Expressao,
        bloco_entao: Vec<Instrucao>,
        bloco_senao: Option<Vec<Instrucao>>,
    },
    Enquanto {
        condicao: Expressao,
        bloco: Vec<Instrucao>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Programa {
    pub instrucoes: Vec<Instrucao>,
}
