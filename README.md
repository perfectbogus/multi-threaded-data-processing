# multi-threaded-data-processing
Rust Project to Learn Multi Threading 

# Architecture
[Input] -> [Parser] -> [Worker Threads] -> [Aggregator] -> [Output]

# Execution
```shell
cargo build
cargo run -- --input sample_data.csv --output processed.csv --normalization 100.0 --threshold 0.9
cargo run -- --input test_input.json --output processed.json --normalization 100.0 --threshold 0.9
```

