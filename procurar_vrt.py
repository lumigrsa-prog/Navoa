import os

for root, dirs, files in os.walk("."):
    for file in files:
        if file.endswith(".vrt"):
            path = os.path.join(root, file)
            print(f"=== {path} ===")
            with open(path, "r", encoding="utf-8") as f:
                print(f.read())
