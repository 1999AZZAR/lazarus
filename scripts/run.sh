#!/bin/bash
set -e

# Build release
echo "Building Lazarus..."
cargo build --release --quiet

BIN="./target/release/lazarus"
DATA_DIR="benchmarks/data"
RESULTS_FILE="benchmarks/results.md"

echo "| File Type | Original Size | Compressed Size | Reduction | Compression Time | Decompression Time |" > $RESULTS_FILE
echo "| :--- | :--- | :--- | :--- | :--- | :--- |" >> $RESULTS_FILE

run_test() {
    FILE=$1
    NAME=$(basename $FILE)
    COMPRESSED="${FILE}.lzr"
    RESTORED="${FILE}.out"
    
    ORIG_SIZE=$(stat -c%s "$FILE")
    
    # Compress
    START_C=$(date +%s%N)
    $BIN compress "$FILE" --output "$COMPRESSED" > /dev/null
    END_C=$(date +%s%N)
    TIME_C=$(( ($END_C - $START_C) / 1000000 )) # ms
    
    COMP_SIZE=$(stat -c%s "$COMPRESSED")
    
    # Decompress
    START_D=$(date +%s%N)
    $BIN decompress "$COMPRESSED" --output "$RESTORED" > /dev/null
    END_D=$(date +%s%N)
    TIME_D=$(( ($END_D - $START_D) / 1000000 )) # ms
    
    # Calc stats using python to avoid awk quoting hell
    REDUCTION=$(python3 -c "print(f'{(1 - $COMP_SIZE/$ORIG_SIZE) * 100:.2f}')")
    TIME_C_SEC=$(python3 -c "print(f'{$TIME_C / 1000:.2f}')")
    TIME_D_SEC=$(python3 -c "print(f'{$TIME_D / 1000:.2f}')")
    
    # Print row
    echo "| **$NAME** | $(numfmt --to=iec $ORIG_SIZE) | $(numfmt --to=iec $COMP_SIZE) | **${REDUCTION}%** | ${TIME_C_SEC}s | ${TIME_D_SEC}s |" >> $RESULTS_FILE
    
    # Cleanup
    rm "$COMPRESSED" "$RESTORED"
}

echo "Running Benchmarks..."
for f in $DATA_DIR/*; do
    echo "Testing $f..."
    run_test "$f"
done

echo "Done! Results saved to $RESULTS_FILE"
cat $RESULTS_FILE