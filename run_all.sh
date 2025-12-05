#!/usr/bin/env bash
files=$(ls src/bin/ | sed 's/\.rs//')
timing_file="timing.txt"
> "$timing_file"
for file in $files; do
    printf "Running $file\n"
    cargo build --release --bin "$file" 2>/dev/null

    start=$(date +%s.%N)
    ./target/release/"$file" &>/dev/null
    end=$(date +%s.%N)
    elapsed=$(awk "BEGIN {printf \"%.4f\", $end - $start}")
    printf "${elapsed}s \n\n"
    printf "$file ${elapsed}s\n" >> "$timing_file"
done
    