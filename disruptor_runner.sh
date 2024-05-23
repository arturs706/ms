#!/bin/bash

echo "Java Execution Time (ms),Rust Execution Time (ms)" > Rust.csv # Header for CSV file
i=0
while [ "$i" -le 100 ]
do
        # Running Java program
        java_output=$(java -jar disruptor-1.0-SNAPSHOT.jar 2>&1)
        java_time=$(echo "$java_output" | awk '/Total execution time:/ {print $4}')

        # Running Rust program
        rust_output=$(cargo run --release --manifest-path=/Users/artursradionovs/Desktop/axum_ms/crossbeamtest/Cargo.toml 2>&1)
        rust_time=$(echo "$rust_output" | awk '/Total execution time:/ {gsub("ms",""); print $4}')

        echo "$java_time,$rust_time" >> Rust.csv
        i=$((i + 1))
done