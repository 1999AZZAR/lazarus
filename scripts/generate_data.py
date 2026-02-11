import os
import json
import csv
import random
import string

def generate_log_line():
    ip = f"{random.randint(1,255)}.{random.randint(0,255)}.{random.randint(0,255)}.{random.randint(0,255)}"
    timestamp = "2026-02-11T12:00:00Z"
    method = random.choice(["GET", "POST", "PUT", "DELETE"])
    url = f"/api/v1/resource/{random.randint(1000,9999)}"
    status = random.choice([200, 201, 400, 404, 500])
    ua = "Mozilla/5.0 (X11; Linux x86_64) Lazarus/1.0"
    return f'{ip} - - [{timestamp}] "{method} {url} HTTP/1.1" {status} 1024 "{ua}"\n'

def generate_json_obj():
    return {
        "id": random.randint(1, 1000000),
        "name": ''.join(random.choices(string.ascii_letters, k=10)),
        "email": ''.join(random.choices(string.ascii_lowercase, k=5)) + "@example.com",
        "tags": [random.choice(["active", "pending", "deleted"]) for _ in range(3)],
        "metadata": {
            "login_count": random.randint(0, 100),
            "is_admin": random.choice([True, False])
        }
    }

def generate_file(path, size_mb, generator_func, is_binary=False):
    print(f"Generating {path} ({size_mb} MB)...")
    target_bytes = size_mb * 1024 * 1024
    written = 0
    with open(path, 'w' if not is_binary else 'wb') as f:
        while written < target_bytes:
            if is_binary:
                # Random binary data (high entropy)
                chunk = os.urandom(min(1024*1024, target_bytes - written))
                f.write(chunk)
                written += len(chunk)
            else:
                data = generator_func()
                if isinstance(data, dict):
                    data = json.dumps(data) + "\n"
                f.write(data)
                written += len(data)

def main():
    os.makedirs("benchmarks/data", exist_ok=True)
    
    # 1. Server Logs (Highly Redundant)
    generate_file("benchmarks/data/server_100mb.log", 100, generate_log_line)
    
    # 2. JSON Data (Structured Text)
    generate_file("benchmarks/data/users_50mb.json", 50, generate_json_obj)
    
    # 3. Binary Data (High Entropy)
    generate_file("benchmarks/data/random_10mb.bin", 10, None, is_binary=True)

if __name__ == "__main__":
    main()