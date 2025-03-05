#!/bin/bash

# Create dist directory if it doesn't exist
mkdir -p dist

# Build the project in release mode
cargo build --target wasm32-unknown-unknown --release

# Check if the build was successful
if [ $? -ne 0 ]; then
    echo "Build failed"
    exit 1
fi

# Path to the compiled wasm file
WASM_FILE="target/wasm32-unknown-unknown/release/sleet_managed_subaccounts.wasm"

# Check if wasm-opt is installed
if ! command -v wasm-opt &> /dev/null; then
    echo "wasm-opt not found. Please install binaryen:"
    echo "brew install binaryen  # for macOS"
    exit 1
fi

# Install cargo-near if not installed
if ! command -v cargo-near &> /dev/null; then
    echo "cargo-near not found. Installing..."
    cargo install cargo-near
fi

# Optimize the wasm file
wasm-opt -Oz -o dist/sleet_managed_subaccounts.wasm $WASM_FILE

# Generate ABI
echo "Generating ABI..."
cargo near abi

# Move the generated ABI file to dist directory
mv target/near/sleet_managed_subaccounts_abi.json dist//sleet_managed_subaccounts_abi.json

echo "Build complete! Optimized WASM file is in dist/sleet_managed_subaccounts.wasm"
echo "ABI file generated at dist/abi.json"