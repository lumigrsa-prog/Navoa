import os

for root, dirs, files in os.walk("crates"):
    for file in files:
        path = os.path.join(root, file)
        try:
            with open(path, "r", encoding="utf-8") as f:
                for i, line in enumerate(f):
                    if "print" in line.lower():
                        print(f"{path}:{i+1}: {line.strip()}")
        except Exception:
            pass
