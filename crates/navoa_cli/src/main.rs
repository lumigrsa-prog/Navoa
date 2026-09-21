use std::fs;
use std::io::{self, Write};
use navoa_lexer::{Lexer, Language};
use navoa_parser::Parser;
use navoa_codegen::CodeGen;
use navoa_vm::VM;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(content) => {
                let codigo_traduzido = traduzir_para_padrao(&content);
                run_script(&codigo_traduzido);
            }
            Err(e) => {
                eprintln!("Erro ao ler o ficheiro '{}': {}", filename, e);
            }
        }
        return;
    }

    println!("=== Navoa REPL ===");
    println!("Comandos úteis:");
    println!("  :bloco       -> Entra no modo de colagem de várias linhas (termina com . numa linha isolada)");
    println!("  :gravar <f>  -> Grava o último bloco num ficheiro");
    println!("  :exec <f>    -> Executa um ficheiro .nav");
    println!("  exit         -> Sai do programa\n");

    let mut vm = VM::new();
    let mut last_block: Vec<String> = Vec::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Erro ao ler entrada");
            break;
        }
        let trimmed = input.trim();
        if trimmed == "exit" || trimmed == "sair" {
            break;
        }

        if trimmed == ":bloco" {
            println!("--- Modo de Bloco Ativo (Escreve '.' numa linha isolada para terminar) ---");
            let mut block_lines = Vec::new();
            loop {
                print!("| ");
                io::stdout().flush().unwrap();
                let mut line = String::new();
                if io::stdin().read_line(&mut line).is_err() {
                    break;
                }
                if line.trim() == "." {
                    break;
                }
                block_lines.push(line);
            }

            if block_lines.is_empty() {
                println!("Bloco vazio.");
                continue;
            }
            last_block = block_lines.clone();
            let full_code = block_lines.concat();
            println!("\nO que deseja fazer com este bloco?");
            println!("1. Executar");
            println!("2. Gravar para ficheiro");
            print!("Escolha (1/2): ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            let choice_trim = choice.trim();

            if choice_trim == "1" || choice_trim.is_empty() {
                let codigo_traduzido = traduzir_para_padrao(&full_code);
                execute_code(&codigo_traduzido, &mut vm);
            } else if choice_trim == "2" {
                print!("Nome do ficheiro (ex: teste.nav): ");
                io::stdout().flush().unwrap();
                let mut fname = String::new();
                io::stdin().read_line(&mut fname).unwrap();
                let fname_clean = fname.trim();
                if let Err(e) = fs::write(fname_clean, &full_code) {
                    eprintln!("Erro ao gravar ficheiro: {}", e);
                } else {
                    println!("Ficheiro '{}' gravado com sucesso!", fname_clean);
                }
            }
            continue;
        }

        if trimmed.starts_with(":gravar ") {
            let filename = trimmed.trim_start_matches(":gravar ").trim();
            if last_block.is_empty() {
                println!("Nenhum bloco recente para gravar. Use :bloco primeiro.");
            } else {
                let full_code = last_block.concat();
                match fs::write(filename, full_code) {
                    Ok(_) => println!("Bloco gravado em '{}'", filename),
                    Err(e) => eprintln!("Erro ao gravar: {}", e),
                }
            }
            continue;
        }

        if trimmed.starts_with(":exec ") {
            let filename = trimmed.trim_start_matches(":exec ").trim();
            match fs::read_to_string(filename) {
                Ok(content) => {
                    let codigo_traduzido = traduzir_para_padrao(&content);
                    run_script(&codigo_traduzido);
                }
                Err(e) => {
                    eprintln!("Erro ao ler ficheiro '{}': {}", filename, e);
                }
            }
            continue;
        }

        if !trimmed.is_empty() {
            let codigo_traduzido = traduzir_para_padrao(&input);
            execute_code(&codigo_traduzido, &mut vm);
        }
    }
}

fn traduzir_para_padrao(codigo: &str) -> String {
    let substituicoes = vec![
        ("print", "imprimir"), ("var", "variavel"), ("if", "se"),
        ("else", "senao"), ("input", "ler"),
        ("variable", "variavel"), ("si", "se"),
        ("sino", "senao"), ("leer", "ler"),
        ("afficher", "imprimir"), ("sinon", "senao"), ("lire", "ler"),
        ("variabile", "variavel"), ("stampa", "imprimir"),
        ("altrimenti", "senao"), ("leggi", "ler"),
        ("drucken", "imprimir"), ("wenn", "se"), ("sonst", "senao"), ("eingabe", "ler"),
    ];

    let mut linhas = Vec::new();
    for linha in codigo.lines() {
        let mut s = linha.to_string();
        for (estrangeiro, padrao) in &substituicoes {
            s = replace_whole_word(&s, estrangeiro, padrao);
        }
        linhas.push(s);
    }
    linhas.join("\n")
}

fn replace_whole_word(text: &str, target: &str, replacement: &str) -> String {
    let mut result = String::new();
    let bytes = text.as_bytes();
    let target_bytes = target.as_bytes();
    let target_len = target_bytes.len();
    let mut i = 0;
    while i < bytes.len() {
        let mut matches = false;
        if i + target_len <= bytes.len() {
            if &bytes[i..i + target_len] == target_bytes {
                let prev_ok = i == 0 || !is_identifier_char(bytes[i - 1]);
                let next_ok = i + target_len == bytes.len() || !is_identifier_char(bytes[i + target_len]);
                if prev_ok && next_ok {
                    matches = true;
                }
            }
        }

        if matches {
            result.push_str(replacement);
            i += target_len;
        } else {
            let ch = text[i..].chars().next().unwrap();
            result.push(ch);
            i += ch.len_utf8();
        }
    }
    result
}

fn is_identifier_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}

fn execute_code(code: &str, vm: &mut VM) {
    let mut lexer = Lexer::new(code, Language::Pt);
    
    match lexer.tokenize() {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens.as_slice());
            match parser.parse() {
                Ok(ast) => {
                    let mut codegen = CodeGen::new();
                    match codegen.compile(ast.as_slice()) {
                        Ok(_) => {
                            if let Err(e) = vm.interpret(ast) {
                                eprintln!("❌ Runtime Error: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Erro de Compilação (CodeGen): {}", e),
                    }
                }
                Err(e) => eprintln!("❌ Erro de Parsing: {}", e),
            }
        }
        Err(e) => eprintln!("❌ Erro Léxico (Lexer): {}", e),
    }
}

fn run_script(code: &str) {
    let mut vm = VM::new();
    execute_code(code, &mut vm);
}
