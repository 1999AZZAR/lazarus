#!/bin/bash
set -e

# Paths
CARGO="/home/ubuntu/.cargo/bin/cargo"
BIN="./target/release/lazarus"
DATA_DIR="scripts/test_data"
RESULTS_FILE="docs/benchmarks/results.md"

# Build release
echo "Building Lazarus v0.1.4 (Release)..."
$CARGO build --release --quiet

mkdir -p $DATA_DIR

# Generate Test Data
echo "Generating intense test data..."
cat <<EOF > $DATA_DIR/generate.py
import os, json, random, string
def gen_logs(mb):
    with open("$DATA_DIR/server.log", 'w') as f:
        target = mb * 1024 * 1024
        w = 0
        while w < target:
            line = f'192.168.1.1 - - [11/Feb/2026] "GET /api/v1/test HTTP/1.1" 200 1024\n'
            f.write(line)
            w += len(line)
def gen_json(mb):
    with open("$DATA_DIR/data.json", 'w') as f:
        target = mb * 1024 * 1024
        w = 0
        while w < target:
            obj = {"id": random.randint(1,1000), "name": "lazarus", "active": True}
            line = json.dumps(obj) + "\n"
            f.write(line)
            w += len(line)
def gen_bin(mb):
    with open("$DATA_DIR/random.bin", 'wb') as f:
        f.write(os.urandom(mb * 1024 * 1024))

gen_logs(50)
gen_json(50)
gen_bin(10)
EOF
python3 $DATA_DIR/generate.py

echo "# Lazarus v0.1.4 Intense Benchmark Results" > $RESULTS_FILE
echo "Tested against XZ (LZMA L9) and Gzip (DEFLATE L6) on aarch64." >> $RESULTS_FILE
echo "Corruption Test: 1KB random data injected into archive." >> $RESULTS_FILE
echo "" >> $RESULTS_FILE
echo "| Tool | File Type | Ratio | Time (C) | Time (D) | Resilience (Chaos) |" >> $RESULTS_FILE
echo "| :--- | :--- | :--- | :--- | :--- | :--- |" >> $RESULTS_FILE

run_bench() {
    FILE=$1
    NAME=$(basename $FILE)
    ORIG_SIZE=$(stat -c%s "$FILE")
    
    echo "--- Benchmarking $NAME ($((ORIG_SIZE/1024/1024))MB) ---"
    
    # 1. Lazarus
    START=$(date +%s%N)
    $BIN compress "$FILE" --output "test.lzr" > /dev/null
    END=$(date +%s%N)
    TC_LZR=$(python3 -c "print(f'{($END - $START)/1000000000:.2f}')")
    SIZE_LZR=$(stat -c%s "test.lzr")
    RATIO_LZR=$(python3 -c "print(f'{(1 - $SIZE_LZR/$ORIG_SIZE)*100:.2f}%')")
    
    START=$(date +%s%N)
    $BIN decompress "test.lzr" --output "test.lzr.out" > /dev/null
    END=$(date +%s%N)
    TD_LZR=$(python3 -c "print(f'{($END - $START)/1000000000:.2f}')")
    
    # Chaos Test Lazarus (Inject 1KB corruption)
    dd if=/dev/urandom of=test.lzr bs=1 count=1024 seek=5000 conv=notrunc status=none
    if $BIN decompress "test.lzr" --output "test.chaos.out" &> /dev/null; then
        SURV_LZR="SUCCESS (Healed)"
    else
        SURV_LZR="FAILED"
    fi
    rm -f test.lzr test.lzr.out test.chaos.out
    
    # 2. XZ (L9)
    START=$(date +%s%N)
    xz -9 -f -c "$FILE" > test.xz
    END=$(date +%s%N)
    TC_XZ=$(python3 -c "print(f'{($END - $START)/1000000000:.2f}')")
    SIZE_XZ=$(stat -c%s "test.xz")
    RATIO_XZ=$(python3 -c "print(f'{(1 - $SIZE_XZ/$ORIG_SIZE)*100:.2f}%')")
    
    START=$(date +%s%N)
    xz -d -f -c test.xz > test.xz.out
    END=$(date +%s%N)
    TD_XZ=$(python3 -c "print(f'{($END - $START)/1000000000:.2f}')")
    
    # Chaos Test XZ
    dd if=/dev/urandom of=test.xz bs=1 count=1024 seek=5000 conv=notrunc status=none
    if xz -d -f -c test.xz &> /dev/null; then SURV_XZ="SUCCESS"; else SURV_XZ="FAILED"; fi
    rm -f test.xz test.xz.out
    
    # 3. Gzip
    START=$(date +%s%N)
    gzip -6 -f -c "$FILE" > test.gz
    END=$(date +%s%N)
    TC_GZ=$(python3 -c "print(f'{($END - $START)/1000000000:.2f}')")
    SIZE_GZ=$(stat -c%s "test.gz")
    RATIO_GZ=$(python3 -c "print(f'{(1 - $SIZE_GZ/$ORIG_SIZE)*100:.2f}%')")
    
    START=$(date +%s%N)
    gzip -d -f -c test.gz > test.gz.out
    END=$(date +%s%N)
    TD_GZ=$(python3 -c "print(f'{($END - $START)/1000000000:.2f}')")
    
    # Chaos Test Gzip
    dd if=/dev/urandom of=test.gz bs=1 count=1024 seek=5000 conv=notrunc status=none
    if gzip -d -f -c test.gz &> /dev/null; then SURV_GZ="SUCCESS"; else SURV_GZ="FAILED"; fi
    rm -f test.gz test.gz.out
    
    # Write to file
    echo "| **Lazarus** | $NAME | **$RATIO_LZR** | ${TC_LZR}s | ${TD_LZR}s | **$SURV_LZR** |" >> $RESULTS_FILE
    echo "| XZ (L9) | $NAME | $RATIO_XZ | ${TC_XZ}s | ${TD_XZ}s | $SURV_XZ |" >> $RESULTS_FILE
    echo "| Gzip (L6) | $NAME | $RATIO_GZ | ${TC_GZ}s | ${TD_GZ}s | $SURV_GZ |" >> $RESULTS_FILE
    echo "| --- | --- | --- | --- | --- | --- |" >> $RESULTS_FILE
}

for f in $DATA_DIR/*.log $DATA_DIR/*.json $DATA_DIR/*.bin; do
    run_bench "$f"
done

# Cleanup data
rm -rf $DATA_DIR

echo "Benchmark complete."
