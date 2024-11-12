# Makefile for building and running the CLI tool

# The default target is 'run'
.PHONY: all build run

# Build the project
build:
	@cargo build --release

# Run the project
run: build
	@./target/release/cloc $(file)

# Clean the build artifacts
clean:
	@cargo clean
