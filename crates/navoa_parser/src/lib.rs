use navoa_ast::{Expressao, Instrucao, Operador, Programa};
use navoa_lexer::{Lexer, Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    token_atual: Token,
}

impl<'a> Parser<'a> {
    pub fn novo(mut lexer: Lexer<'a>) -> Self {
        let token_atual = lexer.proximo_token();
        Self { lexer, token_atual }
    }

    fn avançar(&mut self) {
        self.token_atual = self.lexer.proximo_token();
    }

    pub fn parse_programa(&mut self) -> Programa {
        let mut instrucoes = Vec::new();
        while self.token_atual != Token::EOF {
            if let Some(inst) = self.parse_instrucao() {
                instrucoes.push(inst);
            } else {
                self.avançar();
            }
        }
        Programa { instrucoes }
    }

    fn parse_instrucao(&mut self) -> Option<Instrucao> {
        match self.token_atual.clone() {
            Token::Imprimir => {
                self.avançar();
                let expr = self.parse_expressao()?;
                Some(Instrucao::Imprimir(expr))
            }
            Token::Var => {
                self.avançar();
                if let Token::Identificador(nome) = self.token_atual.clone() {
                    self.avançar();
                    if self.token_atual == Token::Atribuicao {
                        self.avançar();
                        let valor = self.parse_expressao()?;
                        Some(Instrucao::Atribuicao { nome, valor })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Token::Se => {
                self.avançar();
                let condicao = self.parse_expressao()?;
                let bloco_entao = self.parse_bloco()?;
                let bloco_senao = if self.token_atual == Token::Senao {
                    self.avançar();
                    Some(self.parse_bloco()?)
                } else {
                    None
                };
                Some(Instrucao::Se {
                    condicao,
                    bloco_entao,
                    bloco_senao,
                })
            }
            Token::Enquanto => {
                self.avançar();
                let condicao = self.parse_expressao()?;
                let bloco = self.parse_bloco()?;
                Some(Instrucao::Enquanto { condicao, bloco })
            }
            Token::Funcao => {
                self.avançar();
                if let Token::Identificador(nome) = self.token_atual.clone() {
                    self.avançar();
                    if self.token_atual != Token::AbreParenteses {
                        return None;
                    }
                    self.avançar();

                    let mut parametros = Vec::new();
                    while self.token_atual != Token::FechaParenteses && self.token_atual != Token::EOF {
                        if let Token::Identificador(p) = self.token_atual.clone() {
                            parametros.push(p);
                            self.avançar();
                            if self.token_atual == Token::Virgula {
                                self.avançar();
                            }
                        } else {
                            break;
                        }
                    }

                    if self.token_atual == Token::FechaParenteses {
                        self.avançar();
                    }

                    let corpo = self.parse_bloco()?;
                    Some(Instrucao::DeclararFuncao {
                        nome,
                        parametros,
                        corpo,
                    })
                } else {
                    None
                }
            }
            Token::Retornar => {
                self.avançar();
                let expr = self.parse_expressao();
                Some(Instrucao::Retornar(expr))
            }
            _ => {
                if let Some(expr) = self.parse_expressao() {
                    Some(Instrucao::Expressao(expr))
                } else {
                    None
                }
            }
        }
    }

    fn parse_bloco(&mut self) -> Option<Vec<Instrucao>> {
        if self.token_atual != Token::AbreChave {
            return None;
        }
        self.avançar();

        let mut instrucoes = Vec::new();
        while self.token_atual != Token::FechaChave && self.token_atual != Token::EOF {
            if let Some(inst) = self.parse_instrucao() {
                instrucoes.push(inst);
            } else {
                self.avançar();
            }
        }

        if self.token_atual == Token::FechaChave {
            self.avançar();
        }

        Some(instrucoes)
    }

    pub fn parse_expressao(&mut self) -> Option<Expressao> {
        self.parse_igualdade()
    }

    fn parse_igualdade(&mut self) -> Option<Expressao> {
        let mut expr = self.parse_comparacao()?;

        while matches!(self.token_atual, Token::IgualIgual | Token::Diferente) {
            let op = match self.token_atual {
                Token::IgualIgual => Operador::Igual,
                Token::Diferente => Operador::Diferente,
                _ => unreachable!(),
            };
            self.avançar();
            let direita = self.parse_comparacao()?;
            expr = Expressao::Binaria {
                esquerda: Box::new(expr),
                operacao: op,
                direita: Box::new(direita),
            };
        }

        Some(expr)
    }

    fn parse_comparacao(&mut self) -> Option<Expressao> {
        let mut expr = self.parse_adicao()?;

        while matches!(self.token_atual, Token::Menor | Token::Maior | Token::MenorIgual | Token::MaiorIgual) {
            let op = match self.token_atual {
                Token::Menor => Operador::Menor,
                Token::Maior => Operador::Maior,
                Token::MenorIgual => Operador::MenorIgual,
                Token::MaiorIgual => Operador::MaiorIgual,
                _ => unreachable!(),
            };
            self.avançar();
            let direita = self.parse_adicao()?;
            expr = Expressao::Binaria {
                esquerda: Box::new(expr),
                operacao: op,
                direita: Box::new(direita),
            };
        }

        Some(expr)
    }

    fn parse_adicao(&mut self) -> Option<Expressao> {
        let mut expr = self.parse_multiplicacao()?;

        while matches!(self.token_atual, Token::Mais | Token::Menos) {
            let op = match self.token_atual {
                Token::Mais => Operador::Somar,
                Token::Menos => Operador::Subtrair,
                _ => unreachable!(),
            };
            self.avançar();
            let direita = self.parse_multiplicacao()?;
            expr = Expressao::Binaria {
                esquerda: Box::new(expr),
                operacao: op,
                direita: Box::new(direita),
            };
        }

        Some(expr)
    }

    fn parse_multiplicacao(&mut self) -> Option<Expressao> {
        let mut expr = self.parse_primaria()?;

        while matches!(self.token_atual, Token::Asterisco | Token::Barra) {
            let op = match self.token_atual {
                Token::Asterisco => Operador::Multiplicar,
                Token::Barra => Operador::Dividir,
                _ => unreachable!(),
            };
            self.avançar();
            let direita = self.parse_primaria()?;
            expr = Expressao::Binaria {
                esquerda: Box::new(expr),
                operacao: op,
                direita: Box::new(direita),
            };
        }

        Some(expr)
    }

    fn parse_primaria(&mut self) -> Option<Expressao> {
        match self.token_atual.clone() {
            Token::Numero(val) => {
                self.avançar();
                Some(Expressao::Numero(val))
            }
            Token::Texto(txt) => {
                self.avançar();
                Some(Expressao::Texto(txt))
            }
            Token::Verdadeiro => {
                self.avançar();
                Some(Expressao::Booleano(true))
            }
            Token::Falso => {
                self.avançar();
                Some(Expressao::Booleano(false))
            }
            Token::AbreParenteses => {
                self.avançar();
                let expr = self.parse_expressao()?;
                if self.token_atual == Token::FechaParenteses {
                    self.avançar();
                }
                Some(expr)
            }
            Token::Identificador(nome) => {
                self.avançar();
                if self.token_atual == Token::AbreParenteses {
                    self.avançar();
                    let mut argumentos = Vec::new();
                    while self.token_atual != Token::FechaParenteses && self.token_atual != Token::EOF {
                        if let Some(arg) = self.parse_expressao() {
                            argumentos.push(arg);
                            if self.token_atual == Token::Virgula {
                                self.avançar();
                            }
                        } else {
                            break;
                        }
                    }
                    if self.token_atual == Token::FechaParenteses {
                        self.avançar();
                    }
                    Some(Expressao::Chamada { nome, argumentos })
                } else {
                    Some(Expressao::Variavel(nome))
                }
            }
            _ => None,
        }
    }
}
