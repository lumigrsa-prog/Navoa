import sys
import os
import subprocess
import re
from preprocessor import preprocess_code

def validar_sintaxe_pilha(linha):
    erros = []
    clean = re.sub(r'//.*', '', linha).strip()
    if not clean: return erros
    stack, m = [], {')': '(', '}': '{', ']': '['}
    for c in clean:
        if c in "({[": stack.append(c)
        elif c in ")}]":
            if not stack: erros.append(f"Falta abertura para '{c}'")
            else:
                top = stack.pop()
                if top != m[c]: erros.append(f"Tentou fechar '{top}' com '{c}'")
    if stack: erros.append("Delimitador por fechar")
    return erros

def garantir_binario():
    bin_path = os.path.join("target", "debug", "navoa_cli")
    if not os.path.exists(bin_path):
        print("[*] A compilar motor Navoa VM...", flush=True)
        subprocess.run(["cargo", "build", "-p", "navoa_cli", "--quiet"], check=True)
    return bin_path

def modo_interativo():
    print("==================================================", flush=True)
    print("  Navoa Interactive Shell (Bilingue PT/EN)        ", flush=True)
    print("  Escreve 'sair' ou 'exit' para terminar.         ", flush=True)
    print("==================================================", flush=True)
    
    bin_path = garantir_binario()
    buffer_linhas = []
    
    while True:
        try:
            prompt = "... " if buffer_linhas else "navoa> "
            sys.stdout.write(prompt)
            sys.stdout.flush()
            linha = input()
            
            if linha.strip().lower() in ('sair', 'exit', 'quit'):
                print("A sair do ecossistema Navoa. Até logo!", flush=True)
                break
                
            if not linha.strip():
                continue

            erros = validar_sintaxe_pilha(linha)
            if erros:
                sys.stdout.write("\n")
                for err in erros:
                    print(f"⚠️  [Aviso de Sintaxe]: {err}", flush=True)
                print("❌ Comando rejeitado devido a erro de sintaxe.\n", flush=True)
                sys.stdout.flush()
                buffer_linhas = []
                continue

            buffer_linhas.append(linha)
            conteudo_bloco = "\n".join(buffer_linhas)
            
            if linha.strip().endswith((';', '}')) or (len(buffer_linhas) == 1 and not linha.strip().endswith('{')):
                processed = preprocess_code(conteudo_bloco)
                temp_file = "temp_exec.nv"
                with open(temp_file, "w", encoding="utf-8") as out:
                    out.write(processed)
                
                res = subprocess.run([bin_path, temp_file], capture_output=True, text=True)
                if res.returncode != 0:
                    print(f"❌ Erro na Navoa VM: {res.stderr.strip() or res.stdout.strip()}", flush=True)
                else:
                    saida = res.stdout.strip()
                    if saida:
                        print(saida, flush=True)
                    else:
                        print("✓ Ok", flush=True)
                
                buffer_linhas = []
                sys.stdout.flush()
                
        except KeyboardInterrupt:
            print("\nUso: escreve 'sair' para terminar.", flush=True)
            buffer_linhas = []
        except EOFError:
            break

if __name__ == '__main__':
    modo_interativo()
