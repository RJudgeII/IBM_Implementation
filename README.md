# IBM Quantum Systems Exercise Implementation

There are a few things that should be noted before running this repository.

1. This repo assumes both the acme and madrid services are running at the same time, with madrid running on 127.0.0.1:8000 and acme on 127.0.0.1:8001
2. The tests will be run with a simple ```cargo test```
3. The executable will be run with ```cargo run --bin main --release <file_name>```, where <file_name> is a provided JSON file in format similar to those in the example.