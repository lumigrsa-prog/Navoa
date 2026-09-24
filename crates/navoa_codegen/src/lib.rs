use navoa_ast::{Expressao, Instrucao, Programa};

pub struct Codegen;

impl Codegen {
    pub fn gerar_js(programa: &Programa) -> String {
        let mut js = String::new();
        for inst in &programa.instrucoes {
            js.push_str(&Self::gerar_instrucao_js(inst));
        }
        js
    }

    fn gerar_instrucao_js(inst: &Instrucao) -> String {
        match inst {
            Instrucao::Imprimir(expr) => {
                format!("console.log({});\n", Self::gerar_expressao_js(expr))
            }
            Instrucao::Atribuicao { nome, valor } => {
                format!("let {} = {};\n", nome, Self::gerar_expressao_js(valor))
            }
            Instrucao::DeclararFuncao { nome, parametros, corpo } => {
                let params = parametros.join(", ");
                let mut res = format!("function {}({}) {{\n", nome, params);
                for sub in corpo {
                    res.push_str(&format!("  {}", Self::gerar_instrucao_js(sub)));
                }
                res.push_str("}\n");
                res
            }
            Instrucao::Retornar(opt_expr) => match opt_expr {
                Some(expr) => format!("return {};\n", Self::gerar_expressao_js(expr)),
                None => "return;\n".to_string(),
            },
            Instrucao::Se { condicao, bloco_entao, bloco_senao } => {
                let cond = Self::gerar_expressao_js(condicao);
                let mut res = format!("if ({}) {{\n", cond);
                for sub in bloco_entao {
                    res.push_str(&format!("  {}", Self::gerar_instrucao_js(sub)));
                }
                res.push_str("}");
                if let Some(senao) = bloco_senao {
                    res.push_str(" else {\n");
                    for sub in senao {
                        res.push_str(&format!("  {}", Self::gerar_instrucao_js(sub)));
                    }
                    res.push_str("}");
                }
                res.push('\n');
                res
            }
            Instrucao::Enquanto { condicao, bloco } => {
                let cond = Self::gerar_expressao_js(condicao);
                let mut res = format!("while ({}) {{\n", cond);
                for sub in bloco {
                    res.push_str(&format!("  {}", Self::gerar_instrucao_js(sub)));
                }
                res.push_str("}\n");
                res
            }
            Instrucao::Expressao(expr) => {
                format!("{};\n", Self::gerar_expressao_js(expr))
            }
        }
    }

    fn gerar_expressao_js(expr: &Expressao) -> String {
        match expr {
            Expressao::Numero(n) => n.to_string(),
            Expressao::Texto(t) => format!("\"{}\"", t),
            Expressao::Variavel(nome) => nome.clone(),
            Expressao::Booleano(b) => b.to_string(),
            Expressao::Chamada { nome, argumentos } => {
                let args: Vec<String> = argumentos.iter().map(Self::gerar_expressao_js).collect();
                format!("{}({})", nome, args.join(", "))
            }
        }
    }
}
