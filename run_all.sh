#!/usr/bin/env bash
files=$(ls src/bin/ | sed 's/\.rs//')
timing_file="timing.txt"
> "$timing_file"
for file in $files; do
    echo "Running $file"
    cargo build --release --bin "$file" 2>/dev/null

    start=$(date +%s.%N)
    ./target/release/"$file"
    end=$(date +%s.%N)
    elapsed=$(awk "BEGIN {printf \"%.3f\", $end - $start}")
    echo "$file ${elapsed}s" >> "$timing_file"
done
    