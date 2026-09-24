use navoa_lexer::Lexer;
use navoa_parser::Parser;
use navoa_vm::VM;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Uso: navoa <ficheiro.navoa>");
        return;
    }

    let caminho = &args[1];
    let conteudo = fs::read_to_string(caminho).expect("Erro ao ler o ficheiro");

    let lexer = Lexer::novo(&conteudo);
    let mut parser = Parser::novo(lexer);
    let programa = parser.parse_programa();

    let mut vm = VM::nova();
    vm.executar(&programa);
    
    let saida = vm.obter_saida();
    if !saida.is_empty() {
        println!("{}", saida);
    }
}
