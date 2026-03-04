.PHONY: help build test clean format lint check run install dev audit

# Default target
help:
	@echo "Available targets:"
	@echo "  make build     - Build the project in release mode"
	@echo "  make test      - Run all tests"
	@echo "  make clean     - Clean build artifacts"
	@echo "  make format    - Format code with rustfmt"
	@echo "  make lint      - Run clippy linter"
	@echo "  make check     - Run cargo check"
	@echo "  make run       - Run the application"
	@echo "  make install   - Install the binary"
	@echo "  make dev       - Build and run in development mode"
	@echo "  make audit     - Run security audit"
	@echo "  make all       - Format, lint, test, and build"

# Build the project
build:
	cargo build --release

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Format code
format:
	cargo fmt

# Check formatting
format-check:
	cargo fmt --check

# Run clippy
lint:
	cargo clippy -- -D warnings

# Run cargo check
check:
	cargo check

# Run the application
run:
	cargo run

# Install the binary
install:
	cargo install --path .

# Development mode: build and run
dev:
	cargo run

# Security audit
audit:
	cargo audit

# Run all checks
all: format lint test build

# Run CI checks
ci: format-check lint test