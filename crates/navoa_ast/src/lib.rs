#[derive(Debug, PartialEq, Clone)]
pub enum Operador {
    Somar,
    Subtrair,
    Multiplicar,
    Dividir,
    Igual,
    Diferente,
    Menor,
    Maior,
    MenorIgual,
    MaiorIgual,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expressao {
    Numero(f64),
    Texto(String),
    Variavel(String),
    Booleano(bool),
    Chamada {
        nome: String,
        argumentos: Vec<Expressao>,
    },
    Binaria {
        esquerda: Box<Expressao>,
        operacao: Operador,
        direita: Box<Expressao>,
    },
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
    DeclararFuncao {
        nome: String,
        parametros: Vec<String>,
        corpo: Vec<Instrucao>,
    },
    Retornar(Option<Expressao>),
    Expressao(Expressao),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Programa {
    pub instrucoes: Vec<Instrucao>,
}
