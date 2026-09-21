use navoa_lexer::Language;
use navoa_parser::Parser;
use navoa_analyzer::Analyzer;
use navoa_vm::VM;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut lang = Language::Portuguese;

    for i in 1..args.len() {
        if args[i] == "--lang" && i + 1 < args.len() {
            lang = match args[i + 1].to_lowercase().as_str() {
                "en" | "english" => Language::English,
                "es" | "spanish" => Language::Spanish,
                "fr" | "french" => Language::French,
                "it" | "italian" => Language::Italian,
                "de" | "german" => Language::German,
                _ => Language::Portuguese,
            };
        }
    }

    if args.len() > 1 && !args[1].starts_with("--") {
        let filename = &args[1];
        let source = match std::fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Erro ao ler o ficheiro '{}': {}", filename, e);
                std::process::exit(1);
            }
        };
        run_file(&source, lang);
    } else {
        run_repl(lang);
    }
}

fn run_file(source: &str, lang: Language) {
    let mut parser = match Parser::new(source, lang.clone()) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Erro de Parser: {}", e);
            std::process::exit(1);
        }
    };

    let ast = match parser.parse_program() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Erro de Sintaxe: {}", e);
            std::process::exit(1);
        }
    };

    let mut analyzer = Analyzer::new();
    let typed_ast = match analyzer.analyze(ast) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Erro de Análise: {}", e);
            std::process::exit(1);
        }
    };

    let mut vm = VM::new();
    if let Err(e) = vm.interpret(typed_ast) {
        eprintln!("Erro de Execução: {}", e);
        std::process::exit(1);
    }
}

fn run_repl(lang: Language) {
    println!("=== Navoa REPL ===");
    println!("Idioma atual: {:?}.", lang);
    println!("Digite 'ajuda' para ver os comandos ou 'sair' para terminar.\n");

    let mut vm = VM::new();
    let mut buffer = String::new();
    let mut brace_depth: i32 = 0;
    let mut manual_block_mode = false;

    loop {
        if manual_block_mode {
            print_block_prompt(lang.clone());
        } else if brace_depth > 0 {
            print_continuation_prompt(lang.clone());
        } else {
            print_prompt(lang.clone());
        }
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            println!();
            break;
        }

        let trimmed = input.trim();

        if !manual_block_mode && brace_depth == 0 {
            if trimmed.eq_ignore_ascii_case("sair") || trimmed.eq_ignore_ascii_case("exit") {
                println!("Até logo!");
                break;
            }
            if is_help_command(trimmed) {
                print_help(lang.clone());
                continue;
            }
            if trimmed.eq_ignore_ascii_case("bloco") {
                manual_block_mode = true;
                print_block_start_info(lang.clone());
                buffer.clear();
                continue;
            }
        }

        if manual_block_mode {
            if trimmed.eq_ignore_ascii_case("executar") {
                manual_block_mode = false;
                let code_to_run = buffer.clone();
                buffer.clear();
                if !code_to_run.trim().is_empty() {
                    if let Err(err) = execute_code(&code_to_run, &mut vm, lang.clone()) {
                        eprintln!("Erro: {}", err);
                    }
                }
                println!();
                continue;
            }
            if trimmed.eq_ignore_ascii_case("cancelar") {
                manual_block_mode = false;
                buffer.clear();
                println!("Bloco cancelado.\n");
                continue;
            }
            buffer.push_str(&input);
            continue;
        }

        for c in trimmed.chars() {
            if c == '{' {
                brace_depth += 1;
            } else if c == '}' && brace_depth > 0 {
                brace_depth -= 1;
            }
        }

        buffer.push_str(&input);

        if brace_depth == 0 {
            let code_to_run = buffer.clone();
            buffer.clear();
            if !code_to_run.trim().is_empty() {
                if let Err(err) = execute_code(&code_to_run, &mut vm, lang.clone()) {
                    eprintln!("Erro: {}", err);
                }
            }
            println!();
        }
    }
}

fn is_help_command(cmd: &str) -> bool {
    matches!(
        cmd.to_lowercase().as_str(),
        "ajuda" | "help" | "ayuda" | "aide" | "aiuto" | "hilfe"
    )
}

fn print_prompt(lang: Language) {
    match lang {
        Language::Portuguese => print!("navoa (pt)> "),
        Language::English => print!("navoa (en)> "),
        Language::Spanish => print!("navoa (es)> "),
        Language::French => print!("navoa (fr)> "),
        Language::Italian => print!("navoa (it)> "),
        Language::German => print!("navoa (de)> "),
    }
}

fn print_continuation_prompt(lang: Language) {
    match lang {
        Language::Portuguese => print!("navoa (pt)... "),
        Language::English => print!("navoa (en)... "),
        Language::Spanish => print!("navoa (es)... "),
        Language::French => print!("navoa (fr)... "),
        Language::Italian => print!("navoa (it)... "),
        Language::German => print!("navoa (de)... "),
    }
}

fn print_block_prompt(lang: Language) {
    match lang {
        Language::Portuguese => print!("bloco (pt)| "),
        Language::English => print!("block (en)| "),
        Language::Spanish => print!("bloque (es)| "),
        Language::French => print!("bloc (fr)| "),
        Language::Italian => print!("blocco (it)| "),
        Language::German => print!("block (de)| "),
    }
}

fn print_block_start_info(lang: Language) {
    match lang {
        Language::Portuguese => println!("--- Modo de Bloco Manual Ativado ---\nEscreve código livremente. Digita 'executar' numa linha nova para rodar ou 'cancelar' para abortar."),
        Language::English => println!("--- Manual Block Mode Activated ---\nWrite code freely. Type 'executar' on a new line to run or 'cancelar' to abort."),
        Language::Spanish => println!("--- Modo de Bloque Manual Activado ---\nEscribe código libremente. Escribe 'executar' en una nueva línea para ejecutar o 'cancelar' para abortar."),
        Language::French => println!("--- Mode Bloc Manuel Activé ---\nÉcrivez du code librement. Tapez 'executar' sur une nouvelle ligne pour exécuter ou 'cancelar' pour annuler."),
        Language::Italian => println!("--- Modalità Blocco Manuale Attivata ---\nScrivi codice liberamente. Digita 'executar' su una nuova riga per eseguire o 'cancelar' per annullare."),
        Language::German => println!("--- Manueller Block-Modus Aktiviert ---\nSchreibe Code frei. Tippe 'executar' in eine neue Zeile zum Ausführen oder 'cancelar' zum Abbrechen."),
    }
}

fn print_help(lang: Language) {
    match lang {
        Language::Portuguese => {
            println!("--- Comandos Disponíveis ---");
            println!("  sair / exit          - Termina a sessão do REPL.");
            println!("  ajuda / help         - Mostra esta ajuda.");
            println!("  bloco                - Entra no modo de bloco livre multi-linha.");
            println!("Exemplos de código:");
            println!("  var x = 15;");
            println!("  imprimir x + 5;");
            println!("  enquanto (x > 10) {{ imprimir x; x = x - 1; }}");
        }
        Language::English => {
            println!("--- Available Commands ---");
            println!("  sair / exit          - Exit the REPL session.");
            println!("  ajuda / help         - Show this help.");
            println!("  bloco                - Enter multi-line free block mode.");
        }
        _ => {
            println!("--- Commands ---");
            println!("  sair / exit, ajuda / help, bloco");
        }
    }
    println!();
}

fn execute_code(code: &str, vm: &mut VM, lang: Language) -> Result<(), String> {
    let mut parser = Parser::new(code, lang)?;
    let ast = parser.parse_program()?;
    let mut analyzer = Analyzer::new();
    let typed_ast = analyzer.analyze(ast)?;
    vm.interpret(typed_ast)?;
    Ok(())
}
