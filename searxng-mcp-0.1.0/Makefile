.PHONY: build test install clean fmt lint

# Default target
all: build

# Build the project
build:
	cargo build --release

# Build for development
dev:
	cargo build

# Run tests
test:
	cargo test

# Install locally
install: build
	./install.sh

# Clean build artifacts
clean:
	cargo clean

# Format code
fmt:
	cargo fmt

# Run linter
lint:
	cargo clippy -- -D warnings

# Run the server (for testing)
run:
	cargo run

# Check the project
check:
	cargo check
