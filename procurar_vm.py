import os

path = "crates/vortex_vm/src/lib.rs"
if os.path.exists(path):
    with open(path, "r", encoding="utf-8") as f:
        for i, line in enumerate(f):
            if "call" in line.lower() or "getglobal" in line.lower() or "undefined" in line.lower():
                print(f"{i+1}: {line.strip()}")
