import os
import re
import subprocess

def preprocess_code(code):
    # 1. Remover comentários
    code = re.sub(r'//.*', '', code)
    
    # 2. Traduzir built-ins
    builtins_map = {
        r'\btamanho\b': 'len',
        r'\bescrever_ficheiro\b': 'write_file',
        r'\bler_ficheiro\b': 'read_file',
        r'\btipo\b': 'type',
        r'\bnumero\b': 'number',
    }
    for pat, rep in builtins_map.items():
        code = re.sub(pat, rep, code)
        
    # 3. Traduzir palavras-chave
    keywords_map = {
        r'\bseja\b': 'let',
        r'\bsejas\b': 'let',
        r'\bverdadeiro\b': 'true',
        r'\bfalso\b': 'false',
        r'\bsenao\b': 'else',
        r'\bsenão\b': 'else',
        r'\bfuncao\b': 'fn',
        r'\bfunção\b': 'fn',
        r'\bretornar\b': 'return',
        r'\bretorna\b': 'return',
    }
    for pat, rep in keywords_map.items():
        code = re.sub(pat, rep, code)
        
    return code

test_files = [
    "programa.vx",
    "teste_mutacao.vx",
    "teste_avancado.vx",
    "teste_logico.vx",
    "teste_negacao.vx",
    "teste_para.vx",
    "teste_cli.vx",
    "teste_arrays.vx"
]

print("=== A EXECUTAR TODOS OS TESTES DO VORTEX ===")
for test_file in test_files:
    if not os.path.exists(test_file):
        continue
    
    print(f"\n----------------------------------------")
    print(f"[{test_file}] A processar e executar...")
    print(f"----------------------------------------")
    
    with open(test_file, "r", encoding="utf-8") as f:
        content = f.read()
        
    processed = preprocess_code(content)
    
    # Injetar gravação do último estado ou teste no ficheiro de saída por teste
    out_filename = f"out_{test_file}.txt"
    
    with open("temp_exec.vrt", "w", encoding="utf-8") as out:
        out.write(processed)
        
    # Executar via CLI do Vortex
    result = subprocess.run(["cargo", "run", "-p", "vortex_cli", "--", "temp_exec.vrt"], 
                            capture_output=True, text=True)
    
    if result.returncode == 0:
        print(f"Status: Sucesso na VM.")
        if os.path.exists(out_filename):
            with open(out_filename, "r", encoding="utf-8") as rf:
                print("Output:", rf.read())
    else:
        print(f"Erro na execução:")
        print(result.stderr.strip())

print("\n=== TODOS OS TESTES PROCESSADOS ===")
