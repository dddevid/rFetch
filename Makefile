# rFetch Makefile

.PHONY: help build release test clean install uninstall check fmt clippy doc run

# Default target
help:
	@echo "rFetch - Available targets:"
	@echo "  build     - Build in debug mode"
	@echo "  release   - Build in release mode"
	@echo "  test      - Run tests"
	@echo "  check     - Check code without building"
	@echo "  fmt       - Format code"
	@echo "  clippy    - Run clippy linter"
	@echo "  clean     - Clean build artifacts"
	@echo "  install   - Install rFetch system-wide"
	@echo "  uninstall - Remove rFetch from system"
	@echo "  doc       - Generate documentation"
	@echo "  run       - Run in debug mode"
	@echo "  run-release - Run in release mode"

# Build targets
build:
	cargo build

release:
	cargo build --release

# Testing and validation
test:
	cargo test

check:
	cargo check

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

# Documentation
doc:
	cargo doc --open

# Execution
run:
	cargo run

run-release:
	cargo run --release

# Installation
install: release
	@echo "Installing rFetch..."
	@if [ -w /usr/local/bin ]; then \
		cp target/release/rfetch /usr/local/bin/; \
	else \
		sudo cp target/release/rfetch /usr/local/bin/; \
	fi
	@chmod +x /usr/local/bin/rfetch
	@mkdir -p ~/.config/rfetch
	@if [ ! -f ~/.config/rfetch/config.toml ]; then \
		cp config.example.toml ~/.config/rfetch/config.toml; \
	fi
	@echo "rFetch installed successfully!"

uninstall:
	@echo "Removing rFetch..."
	@if [ -w /usr/local/bin/rfetch ]; then \
		rm -f /usr/local/bin/rfetch; \
	else \
		sudo rm -f /usr/local/bin/rfetch; \
	fi
	@echo "rFetch removed successfully!"

# Cleanup
clean:
	cargo clean

# Development workflow
dev: fmt clippy test

# All checks before commit
pre-commit: fmt clippy test check

# Package for distribution
package: clean release
	@echo "Creating package..."
	@mkdir -p dist
	@cp target/release/rfetch dist/
	@cp README.md dist/
	@cp LICENSE dist/
	@cp config.example.toml dist/
	@tar -czf dist/rfetch-$(shell cargo metadata --format-version 1 | jq -r '.packages[0].version').tar.gz -C dist .
	@echo "Package created in dist/"