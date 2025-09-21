#!/bin/bash

set -e

# Build the binary in release mode
cargo build --release

# Define paths
BINARY_NAME="verse"
BINARY_PATH="target/release/$BINARY_NAME"
RELEASE_DIR="release/latest"
TARBALL_NAME="$BINARY_NAME.tar.gz"

# Create release directory if it doesn't exist
mkdir -p "$RELEASE_DIR"

# Copy the binary to a temp location for packaging
cp "$BINARY_PATH" "$RELEASE_DIR/$BINARY_NAME"

# Create the tarball (overwrite if exists)
tar -czf "$RELEASE_DIR/$TARBALL_NAME" -C "$RELEASE_DIR" "$BINARY_NAME"

# Remove the copied binary, keep only the tarball
rm "$RELEASE_DIR/$BINARY_NAME"

echo "Tarball created at $RELEASE_DIR/$TARBALL_NAME"
