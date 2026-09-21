if [ -z "$1" ]; then
    echo "Uso: bash run.sh <ficheiro.vx>"
    exit 1
fi

rm -f stdout.txt temp_exec.vrt
python3 preprocessor.py "$1"

if [ $? -eq 0 ]; then
    echo "--- A executar na VM do Vortex ---"
    cargo run -p vortex_cli -- temp_exec.vrt
    
    if [ -f "stdout.txt" ]; then
        echo "--- Output Capturado ---"
        cat stdout.txt
        echo ""
    fi
fi
