#!/bin/bash

# Usage: ./install-verse.sh <github_tarball_url>
# Example: ./install-verse.sh https://github.com/username/repo/releases/download/v0.1.0/verse.tar.gz

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <github_tarball_url>"
    exit 1
fi

TAR_URL="$1"
TMP_DIR=$(mktemp -d)

# Download the tarball
curl -L "$TAR_URL" -o "$TMP_DIR/verse.tar.gz"

# Extract the tarball
cd "$TMP_DIR"
tar -xzf verse.tar.gz

# Find the binary (assumes it's named 'verse')
if [ ! -f "verse" ]; then
    echo "verse binary not found in the tarball."
    exit 1
fi

# Move the binary to ~/.local/bin (preferred) or ~/bin as fallback
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"
mv verse "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/verse"

echo "verse installed to $INSTALL_DIR. Make sure $INSTALL_DIR is in your PATH."

# Clean up
tmp_cleanup() {
    rm -rf "$TMP_DIR"
}
trap tmp_cleanup EXIT
