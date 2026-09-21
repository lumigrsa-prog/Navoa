use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};

// ==========================================
// 1. LEXER (Analisador Léxico)
// ==========================================

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Seja,
    Imprimir,
    Se,
    Senao,
    Enquanto,
    Funcao,
    Retorna,
    Ler,
    Identificador(String),
    Numero(f64),
    Texto(String),
    Verdadeiro,
    Falso,
    Atribuir,          // =
    Mais,              // +
    Menos,             // -
    Multiplicar,       // *
    Dividir,           // /
    IgualIgual,        // ==
    Diferente,         // !=
    Menor,             // <
    MenorIgual,        // <=
    Maior,             // >
    MaiorIgual,        // >=
    E,                 // &&
    Ou,                // ||
    Nao,               // !
    ParentesisEsq,     // (
    ParentesisDir,     // )
    ChavetaEsq,        // {
    ChavetaDir,        // }
    Virgula,           // ,
    FimDeLinha,
    EOF,
}

struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Lexer {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<&char> {
        self.chars.get(self.pos)
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).copied();
        self.pos += 1;
        c
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(&c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else if c == '/' && self.chars.get(self.pos + 1) == Some(&'/') {
                while let Some(&nc) = self.peek() {
                    self.advance();
                    if nc == '\n' { break; }
                }
            } else if c.is_ascii_alphabetic() || c == '_' {
                let mut ident = String::new();
                while let Some(&nc) = self.peek() {
                    if nc.is_ascii_alphanumeric() || nc == '_' {
                        ident.push(self.advance().unwrap());
                    } else {
                        break;
                    }
                }
                match ident.as_str() {
                    "seja" => tokens.push(Token::Seja),
                    "imprimir" => tokens.push(Token::Imprimir),
                    "se" => tokens.push(Token::Se),
                    "senao" => tokens.push(Token::Senao),
                    "enquanto" => tokens.push(Token::Enquanto),
                    "funcao" => tokens.push(Token::Funcao),
                    "retorna" => tokens.push(Token::Retorna),
                    "ler" => tokens.push(Token::Ler),
                    "verdadeiro" => tokens.push(Token::Verdadeiro),
                    "falso" => tokens.push(Token::Falso),
                    _ => tokens.push(Token::Identificador(ident)),
                }
            } else if c.is_ascii_digit() {
                let mut num_str = String::new();
                while let Some(&nc) = self.peek() {
                    if nc.is_ascii_digit() || nc == '.' {
                        num_str.push(self.advance().unwrap());
                    } else {
                        break;
                    }
                }
                let val: f64 = num_str.parse().unwrap_or(0.0);
                tokens.push(Token::Numero(val));
            } else if c == '"' {
                self.advance();
                let mut text = String::new();
                while let Some(&nc) = self.peek() {
                    if nc == '"' {
                        self.advance();
                        break;
                    } else {
                        text.push(self.advance().unwrap());
                    }
                }
                tokens.push(Token::Texto(text));
            } else {
                match c {
                    '=' => {
                        self.advance();
                        if self.peek() == Some(&'=') {
                            self.advance();
                            tokens.push(Token::IgualIgual);
                        } else {
                            tokens.push(Token::Atribuir);
                        }
                    }
                    '!' => {
                        self.advance();
                        if self.peek() == Some(&'=') {
                            self.advance();
                            tokens.push(Token::Diferente);
                        } else {
                            tokens.push(Token::Nao);
                        }
                    }
                    '<' => {
                        self.advance();
                        if self.peek() == Some(&'=') {
                            self.advance();
                            tokens.push(Token::MenorIgual);
                        } else {
                            tokens.push(Token::Menor);
                        }
                    }
                    '>' => {
                        self.advance();
                        if self.peek() == Some(&'=') {
                            self.advance();
                            tokens.push(Token::MaiorIgual);
                        } else {
                            tokens.push(Token::Maior);
                        }
                    }
                    '&' => {
                        self.advance();
                        if self.peek() == Some(&'&') {
                            self.advance();
                            tokens.push(Token::E);
                        }
                    }
                    '|' => {
                        self.advance();
                        if self.peek() == Some(&'|') {
                            self.advance();
                            tokens.push(Token::Ou);
                        }
                    }
                    '+' => { self.advance(); tokens.push(Token::Mais); }
                    '-' => { self.advance(); tokens.push(Token::Menos); }
                    '*' => { self.advance(); tokens.push(Token::Multiplicar); }
                    '/' => { self.advance(); tokens.push(Token::Dividir); }
                    '(' => { self.advance(); tokens.push(Token::ParentesisEsq); }
                    ')' => { self.advance(); tokens.push(Token::ParentesisDir); }
                    '{' => { self.advance(); tokens.push(Token::ChavetaEsq); }
                    '}' => { self.advance(); tokens.push(Token::ChavetaDir); }
                    ',' => { self.advance(); tokens.push(Token::Virgula); }
                    '\n' => { self.advance(); tokens.push(Token::FimDeLinha); }
                    _ => { self.advance(); }
                }
            }
        }
        tokens.push(Token::EOF);
        tokens
    }
}

// ==========================================
// 2. AST (Árvore Sintática)
// ==========================================

#[derive(Debug, Clone)]
enum Expr {
    Numero(f64),
    Texto(String),
    Booleano(bool),
    Variavel(String),
    Chamada(String, Vec<Expr>),
    Ler,
    BinOp(Box<Expr>, Op, Box<Expr>),
    UnOp(OpUnaria, Box<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Mais, Menos, Multiplicar, Dividir,
    IgualIgual, Diferente, Menor, MenorIgual, Maior, MaiorIgual,
    E, Ou,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum OpUnaria {
    Nao, Menos,
}

#[derive(Debug, Clone)]
enum Stmt {
    Atribuicao(String, Expr),
    Imprimir(Expr),
    Se(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    Enquanto(Expr, Vec<Stmt>),
    Funcao(String, Vec<String>, Vec<Stmt>),
    Retorna(Option<Expr>),
}

// ==========================================
// 3. PARSER (Analisador Sintático)
// ==========================================

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) -> Token {
        let t = self.peek().clone();
        self.pos += 1;
        t
    }

    fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while self.peek() != &Token::EOF {
            if self.peek() == &Token::FimDeLinha {
                self.advance();
                continue;
            }
            statements.push(self.parse_stmt());
        }
        statements
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.peek() {
            Token::Seja => {
                self.advance();
                let nome = match self.advance() {
                    Token::Identificador(s) => s,
                    _ => "".to_string(),
                };
                if self.peek() == &Token::Atribuir {
                    self.advance();
                }
                let expr = self.parse_expr();
                Stmt::Atribuicao(nome, expr)
            }
            Token::Imprimir => {
                self.advance();
                if self.peek() == &Token::ParentesisEsq { self.advance(); }
                let expr = self.parse_expr();
                if self.peek() == &Token::ParentesisDir { self.advance(); }
                Stmt::Imprimir(expr)
            }
            Token::Se => {
                self.advance();
                let cond = self.parse_expr();
                if self.peek() == &Token::ChavetaEsq { self.advance(); }

                let mut corpo_se = Vec::new();
                while self.peek() != &Token::ChavetaDir && self.peek() != &Token::EOF {
                    if self.peek() == &Token::FimDeLinha { self.advance(); continue; }
                    corpo_se.push(self.parse_stmt());
                }
                if self.peek() == &Token::ChavetaDir { self.advance(); }

                let mut corpo_senao = None;
                if self.peek() == &Token::Senao {
                    self.advance();
                    if self.peek() == &Token::ChavetaEsq { self.advance(); }
                    let mut senao_stmts = Vec::new();
                    while self.peek() != &Token::ChavetaDir && self.peek() != &Token::EOF {
                        if self.peek() == &Token::FimDeLinha { self.advance(); continue; }
                        senao_stmts.push(self.parse_stmt());
                    }
                    if self.peek() == &Token::ChavetaDir { self.advance(); }
                    corpo_senao = Some(senao_stmts);
                }

                Stmt::Se(cond, corpo_se, corpo_senao)
            }
            Token::Enquanto => {
                self.advance();
                let cond = self.parse_expr();
                if self.peek() == &Token::ChavetaEsq { self.advance(); }

                let mut corpo = Vec::new();
                while self.peek() != &Token::ChavetaDir && self.peek() != &Token::EOF {
                    if self.peek() == &Token::FimDeLinha { self.advance(); continue; }
                    corpo.push(self.parse_stmt());
                }
                if self.peek() == &Token::ChavetaDir { self.advance(); }

                Stmt::Enquanto(cond, corpo)
            }
            Token::Funcao => {
                self.advance();
                let nome = match self.advance() {
                    Token::Identificador(s) => s,
                    _ => "".to_string(),
                };
                if self.peek() == &Token::ParentesisEsq { self.advance(); }
                let mut params = Vec::new();
                while self.peek() != &Token::ParentesisDir && self.peek() != &Token::EOF {
                    if let Token::Identificador(p) = self.advance() {
                        params.push(p);
                    }
                    if self.peek() == &Token::Virgula { self.advance(); }
                }
                if self.peek() == &Token::ParentesisDir { self.advance(); }
                if self.peek() == &Token::ChavetaEsq { self.advance(); }

                let mut corpo = Vec::new();
                while self.peek() != &Token::ChavetaDir && self.peek() != &Token::EOF {
                    if self.peek() == &Token::FimDeLinha { self.advance(); continue; }
                    corpo.push(self.parse_stmt());
                }
                if self.peek() == &Token::ChavetaDir { self.advance(); }

                Stmt::Funcao(nome, params, corpo)
            }
            Token::Retorna => {
                self.advance();
                let expr = if self.peek() != &Token::FimDeLinha && self.peek() != &Token::ChavetaDir && self.peek() != &Token::EOF {
                    Some(self.parse_expr())
                } else {
                    None
                };
                Stmt::Retorna(expr)
            }
            _ => {
                self.advance();
                Stmt::Imprimir(Expr::Numero(0.0))
            }
        }
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Expr {
        let mut expr = self.parse_and();
        while self.peek() == &Token::Ou {
            self.advance();
            let right = self.parse_and();
            expr = Expr::BinOp(Box::new(expr), Op::Ou, Box::new(right));
        }
        expr
    }

    fn parse_and(&mut self) -> Expr {
        let mut expr = self.parse_equality();
        while self.peek() == &Token::E {
            self.advance();
            let right = self.parse_equality();
            expr = Expr::BinOp(Box::new(expr), Op::E, Box::new(right));
        }
        expr
    }

    fn parse_equality(&mut self) -> Expr {
        let mut expr = self.parse_comparison();
        loop {
            let op = match self.peek() {
                Token::IgualIgual => Op::IgualIgual,
                Token::Diferente => Op::Diferente,
                _ => break,
            };
            self.advance();
            let right = self.parse_comparison();
            expr = Expr::BinOp(Box::new(expr), op, Box::new(right));
        }
        expr
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_term();
        loop {
            let op = match self.peek() {
                Token::Menor => Op::Menor,
                Token::MenorIgual => Op::MenorIgual,
                Token::Maior => Op::Maior,
                Token::MaiorIgual => Op::MaiorIgual,
                _ => break,
            };
            self.advance();
            let right = self.parse_term();
            expr = Expr::BinOp(Box::new(expr), op, Box::new(right));
        }
        expr
    }

    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();
        loop {
            let op = match self.peek() {
                Token::Mais => Op::Mais,
                Token::Menos => Op::Menos,
                _ => break,
            };
            self.advance();
            let right = self.parse_factor();
            expr = Expr::BinOp(Box::new(expr), op, Box::new(right));
        }
        expr
    }

    fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_unary();
        loop {
            let op = match self.peek() {
                Token::Multiplicar => Op::Multiplicar,
                Token::Dividir => Op::Dividir,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary();
            expr = Expr::BinOp(Box::new(expr), op, Box::new(right));
        }
        expr
    }

    fn parse_unary(&mut self) -> Expr {
        if self.peek() == &Token::Nao {
            self.advance();
            Expr::UnOp(OpUnaria::Nao, Box::new(self.parse_unary()))
        } else if self.peek() == &Token::Menos {
            self.advance();
            Expr::UnOp(OpUnaria::Menos, Box::new(self.parse_unary()))
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Expr {
        match self.advance() {
            Token::Numero(n) => Expr::Numero(n),
            Token::Texto(s) => Expr::Texto(s),
            Token::Verdadeiro => Expr::Booleano(true),
            Token::Falso => Expr::Booleano(false),
            Token::Ler => {
                if self.peek() == &Token::ParentesisEsq { self.advance(); }
                if self.peek() == &Token::ParentesisDir { self.advance(); }
                Expr::Ler
            }
            Token::Identificador(s) => {
                if self.peek() == &Token::ParentesisEsq {
                    self.advance();
                    let mut args = Vec::new();
                    while self.peek() != &Token::ParentesisDir && self.peek() != &Token::EOF {
                        args.push(self.parse_expr());
                        if self.peek() == &Token::Virgula { self.advance(); }
                    }
                    if self.peek() == &Token::ParentesisDir { self.advance(); }
                    Expr::Chamada(s, args)
                } else {
                    Expr::Variavel(s)
                }
            }
            Token::ParentesisEsq => {
                let expr = self.parse_expr();
                if self.peek() == &Token::ParentesisDir {
                    self.advance();
                }
                expr
            }
            _ => Expr::Numero(0.0),
        }
    }
}

// ==========================================
// 4. MÁQUINA VIRTUAL / INTERPRETADOR
// ==========================================

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Numero(f64),
    Texto(String),
    Booleano(bool),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Numero(n) => write!(f, "{}", n),
            Value::Texto(s) => write!(f, "{}", s),
            Value::Booleano(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Clone)]
enum Flow {
    Normal,
    Return(Value),
}

struct Interpreter {
    env: HashMap<String, Value>,
    functions: HashMap<String, (Vec<String>, Vec<Stmt>)>,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            env: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    fn interpret(&mut self, statements: Vec<Stmt>) {
        for stmt in statements {
            self.execute_stmt(stmt);
        }
    }

    fn execute_stmt(&mut self, stmt: Stmt) -> Flow {
        match stmt {
            Stmt::Atribuicao(name, expr) => {
                let val = self.eval_expr(expr);
                self.env.insert(name, val);
                Flow::Normal
            }
            Stmt::Imprimir(expr) => {
                let val = self.eval_expr(expr);
                println!("{}", val);
                Flow::Normal
            }
            Stmt::Se(cond, corpo_se, corpo_senao) => {
                let res = self.eval_expr(cond);
                let executar_se = match res {
                    Value::Booleano(b) => b,
                    Value::Numero(n) => n != 0.0,
                    Value::Texto(s) => !s.is_empty(),
                };

                if executar_se {
                    for s in corpo_se {
                        match self.execute_stmt(s) {
                            Flow::Return(v) => return Flow::Return(v),
                            Flow::Normal => {}
                        }
                    }
                } else if let Some(senao_stmts) = corpo_senao {
                    for s in senao_stmts {
                        match self.execute_stmt(s) {
                            Flow::Return(v) => return Flow::Return(v),
                            Flow::Normal => {}
                        }
                    }
                }
                Flow::Normal
            }
            Stmt::Enquanto(cond, corpo) => {
                loop {
                    let res = self.eval_expr(cond.clone());
                    let continuar = match res {
                        Value::Booleano(b) => b,
                        Value::Numero(n) => n != 0.0,
                        Value::Texto(s) => !s.is_empty(),
                    };

                    if !continuar { break; }

                    for s in corpo.clone() {
                        match self.execute_stmt(s) {
                            Flow::Return(v) => return Flow::Return(v),
                            Flow::Normal => {}
                        }
                    }
                }
                Flow::Normal
            }
            Stmt::Funcao(nome, params, corpo) => {
                self.functions.insert(nome, (params, corpo));
                Flow::Normal
            }
            Stmt::Retorna(expr) => {
                let val = match expr {
                    Some(e) => self.eval_expr(e),
                    None => Value::Numero(0.0),
                };
                Flow::Return(val)
            }
        }
    }

    fn call_function(&mut self, name: &str, args: Vec<Expr>) -> Value {
        let (params, corpo) = match self.functions.get(name) {
            Some(f) => (f.0.clone(), f.1.clone()),
            None => {
                println!("Erro: Função '{}' não encontrada.", name);
                return Value::Numero(0.0);
            }
        };

        let evaluated_args: Vec<Value> = args.into_iter().map(|arg| self.eval_expr(arg)).collect();

        let mut local_env = self.env.clone();
        for (param, val) in params.into_iter().zip(evaluated_args) {
            local_env.insert(param, val);
        }

        let old_env = std::mem::replace(&mut self.env, local_env);
        let mut ret_val = Value::Numero(0.0);

        for stmt in corpo {
            match self.execute_stmt(stmt) {
                Flow::Return(v) => {
                    ret_val = v;
                    break;
                }
                Flow::Normal => {}
            }
        }

        self.env = old_env;
        ret_val
    }

    fn eval_expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Numero(n) => Value::Numero(n),
            Expr::Texto(s) => Value::Texto(s),
            Expr::Booleano(b) => Value::Booleano(b),
            Expr::Variavel(name) => {
                self.env.get(&name).cloned().unwrap_or(Value::Numero(0.0))
            }
            Expr::Ler => {
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                Value::Texto(input.trim().to_string())
            }
            Expr::Chamada(name, args) => {
                self.call_function(&name, args)
            }
            Expr::UnOp(op, expr) => {
                let val = self.eval_expr(*expr);
                match (op, val) {
                    (OpUnaria::Nao, Value::Booleano(b)) => Value::Booleano(!b),
                    (OpUnaria::Menos, Value::Numero(n)) => Value::Numero(-n),
                    _ => Value::Numero(0.0),
                }
            }
            Expr::BinOp(left, op, right) => {
                let l = self.eval_expr(*left);
                let r = self.eval_expr(*right);

                match (l, op, r) {
                    (Value::Numero(a), Op::Mais, Value::Numero(b)) => Value::Numero(a + b),
                    (Value::Texto(a), Op::Mais, Value::Texto(b)) => Value::Texto(format!("{}{}", a, b)),
                    (Value::Numero(a), Op::Menos, Value::Numero(b)) => Value::Numero(a - b),
                    (Value::Numero(a), Op::Multiplicar, Value::Numero(b)) => Value::Numero(a * b),
                    (Value::Numero(a), Op::Dividir, Value::Numero(b)) => Value::Numero(a / b),

                    (Value::Numero(a), Op::IgualIgual, Value::Numero(b)) => Value::Booleano(a == b),
                    (Value::Texto(a), Op::IgualIgual, Value::Texto(b)) => Value::Booleano(a == b),
                    (Value::Booleano(a), Op::IgualIgual, Value::Booleano(b)) => Value::Booleano(a == b),

                    (Value::Numero(a), Op::Diferente, Value::Numero(b)) => Value::Booleano(a != b),
                    (Value::Texto(a), Op::Diferente, Value::Texto(b)) => Value::Booleano(a != b),

                    (Value::Numero(a), Op::Menor, Value::Numero(b)) => Value::Booleano(a < b),
                    (Value::Numero(a), Op::MenorIgual, Value::Numero(b)) => Value::Booleano(a <= b),
                    (Value::Numero(a), Op::Maior, Value::Numero(b)) => Value::Booleano(a > b),
                    (Value::Numero(a), Op::MaiorIgual, Value::Numero(b)) => Value::Booleano(a >= b),

                    (Value::Booleano(a), Op::E, Value::Booleano(b)) => Value::Booleano(a && b),
                    (Value::Booleano(a), Op::Ou, Value::Booleano(b)) => Value::Booleano(a || b),

                    _ => Value::Numero(0.0),
                }
            }
        }
    }
}

// ==========================================
// 5. MAIN
// ==========================================

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Uso: navoa_cli <ficheiro.nv>");
        return;
    }

    let filename = &args[1];
    let code = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(_) => {
            println!("Erro ao ler o ficheiro: {}", filename);
            return;
        }
    };

    let mut lexer = Lexer::new(&code);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let statements = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.interpret(statements);
}
