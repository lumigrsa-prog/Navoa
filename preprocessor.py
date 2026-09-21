import re
import sys

def preprocess_code(code):
    # 1. Remover comentários de linha única
    code = re.sub(r'//.*', '', code)
    
    # 2. Traduzir funções nativas e atalhos de saída, adicionando parênteses se omitidos
    # Ex: imprimir "texto" -> print("texto")
    builtins_map = {
        r'\btamanho\b': 'len',
        r'\bmostrar\b': 'print',
        r'\bexibir\b': 'print',
        r'\bimprimir\b': 'print',
    }
    
    for pat, rep in builtins_map.items():
        # Se já tiver parênteses, apenas substitui a palavra-chave
        code = re.sub(pat + r'\s*\(', f'{rep}(', code)
        # Se não tiver parênteses, captura o argumento até ao ponto e vírgula ou fim de linha e envolve em parênteses
        code = re.sub(pat + r'\s+([^;\n]+)(;?)', f'{rep}(\\1)\\2', code)

    # 3. Traduzir palavras-chave bilíngues estruturais
    keywords_map = {
        r'\bseja\b': 'let',
        r'\bsejas\b': 'let',
        r'\bse\b': 'if',
        r'\bsenao\b': 'else',
        r'\bsenão\b': 'else',
        r'\benquanto\b': 'while',
        r'\bfuncao\b': 'fn',
        r'\bfunção\b': 'fn',
        r'\bretornar\b': 'return',
        r'\bretorna\b': 'return',
        r'\bverdadeiro\b': 'true',
        r'\bfalso\b': 'false',
    }
    for pattern, replacement in keywords_map.items():
        code = re.sub(pattern, replacement, code)

    return code

if __name__ == '__main__':
    if len(sys.argv) > 1:
        filepath = sys.argv[1]
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        processed = preprocess_code(content)
        with open("temp_exec.vrt", "w", encoding="utf-8") as out:
            out.write(processed)
