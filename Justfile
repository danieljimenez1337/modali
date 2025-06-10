# List cmds
default:
  @just --list

# Run for dev
run:
  cargo run -- -i bindings.json

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
