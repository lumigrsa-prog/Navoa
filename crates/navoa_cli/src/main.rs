use navoa_lexer::Lexer;
use navoa_parser::Parser;
use navoa_vm::VM;
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let caminho = &args[1];
        let conteudo = fs::read_to_string(caminho).expect("Erro ao ler o ficheiro");
        executar_codigo(&conteudo, &mut VM::nova());
    } else {
        iniciar_repl();
    }
}

fn iniciar_repl() {
    println!("=== Navoa REPL (v0.1.0) ===");
    println!("Escreva o seu código em PT, EN, FR, ES, IT ou DE.");
    println!("Digite 'sair' ou 'exit' para fechar.\n");

    let mut vm = VM::nova();
    let stdin = io::stdin();

    loop {
        print!("navoa> ");
        io::stdout().flush().unwrap();

        let mut linha = String::new();
        if stdin.read_line(&mut linha).is_err() || linha.trim() == "sair" || linha.trim() == "exit" {
            println!("\nAté à próxima!");
            break;
        }

        if linha.trim().is_empty() {
            continue;
        }

        executar_codigo(&linha, &mut vm);
    }
}

fn executar_codigo(codigo: &str, vm: &mut VM) {
    let lexer = Lexer::novo(codigo);
    let mut parser = Parser::novo(lexer);
    let programa = parser.parse_programa();

    vm.executar(&programa);

    let saida = vm.obter_saida();
    if !saida.is_empty() {
        println!("{}", saida);
    }
}
