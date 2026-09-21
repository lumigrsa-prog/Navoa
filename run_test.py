import subprocess, os

if os.path.exists("resultado.db"):
    os.remove("resultado.db")

prep_res = subprocess.run(["python3", "preprocessor.py", "teste_db.vx"], capture_output=True, text=True)

if prep_res.returncode == 0 and os.path.exists("temp_exec.vrt"):
    res = subprocess.run(["cargo", "run", "-p", "vortex_cli", "--", "temp_exec.vrt"], capture_output=True, text=True)
    
    if res.returncode == 0:
        print("=== EXECUÇÃO BEM-SUCEDIDA NA VM VORTEX! ===")
        if os.path.exists("resultado.db"):
            print("Ficheiro de base de dados gerado com sucesso em 'resultado.db'.")
        else:
            print("Script executado com sucesso.")
    else:
        print("Erro na VM:")
        print(res.stderr.strip())
else:
    print("Erro no pré-processador:", prep_res.stderr.strip())
