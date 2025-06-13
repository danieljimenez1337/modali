# List cmds
default:
  @just --list

# Run for dev json
run:
  cargo run -- -i bindings.json

# Run for dev ron
run_ron:
  cargo run -- -i bindings.ron

# Run tests
test:
  cargo test

# Lint
lint:
  cargo clippy

# Format
format:
  cargo fmt

# Format Check
format_check:
  cargo fmt --check
