import os
import subprocess
import glob
import sqlite3

def run_tests():
    test_files = glob.glob("tests/*.vx")
    if not test_files:
        print("Nenhum teste encontrado na pasta 'tests/'.")
        return

    passed = 0
    failed = 0

    print(f"[*] A executar {len(test_files)} testes automatizados via SQLite...")
    print("-" * 50)

    for test_file in sorted(test_files):
        base_name = os.path.splitext(test_file)[0]
        exp_file = base_name + ".exp"
        
        # Limpar base de dados de teste anterior
        if os.path.exists("test_res.db"):
            os.remove("test_res.db")
        if os.path.exists("temp_exec.vrt"):
            os.remove("temp_exec.vrt")

        # 1. Pré-processar o ficheiro .vx
        prep_res = subprocess.run(["python3", "preprocessor.py", test_file], capture_output=True, text=True)
        if prep_res.returncode != 0:
            print(f"[FALHA] {test_file} -> Erro no pré-processador")
            print(prep_res.stderr.strip())
            failed += 1
            continue

        # 2. Executar a VM do Vortex via Cargo
        res = subprocess.run(["cargo", "run", "-p", "vortex_cli", "--", "temp_exec.vrt"], capture_output=True, text=True)
        if res.returncode != 0:
            print(f"[FALHA] {test_file} -> Erro na execução da VM")
            print(res.stderr.strip())
            failed += 1
            continue

        # 3. Ler o resultado a partir da base de dados SQLite gerada pela VM
        output_content = ""
        if os.path.exists("test_res.db"):
            try:
                conn = sqlite3.connect("test_res.db")
                cursor = conn.cursor()
                cursor.execute("SELECT val FROM resultado;")
                row = cursor.fetchone()
                if row:
                    output_content = str(row[0])
                conn.close()
            except Exception as e:
                output_content = f"Erro SQL: {e}"

        # 4. Validar contra o ficheiro esperado (.exp)
        test_success = True
        if os.path.exists(exp_file):
            with open(exp_file, "r", encoding="utf-8") as f:
                expected_content = f.read().strip()
            
            if output_content != expected_content:
                print(f"[FALHA] {test_file}")
                print(f"  Esperado: {expected_content!r}")
                print(f"  Obtido:   {output_content!r}")
                test_success = False

        if test_success:
            print(f"[SUCESSO] {test_file}")
            passed += 1
        else:
            failed += 1

    print("-" * 50)
    print(f"Resumo: {passed} testes com sucesso, {failed} falhas.")

    # Limpeza final
    for f in ["test_res.db", "temp_exec.vrt"]:
        if os.path.exists(f):
            os.remove(f)

if __name__ == "__main__":
    run_tests()
