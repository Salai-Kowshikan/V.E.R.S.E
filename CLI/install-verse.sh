#!/bin/bash


# Usage: ./install-verse.sh
# Downloads and installs the latest verse binary from the official GitHub repository.

set -e

TAR_URL="https://raw.githubusercontent.com/Salai-Kowshikan/V.E.R.S.E/main/CLI/release/latest/verse.tar.gz"
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
echo "verse installed to $INSTALL_DIR. Make sure $INSTALL_DIR is in your PATH."
# Move the binary to /usr/bin (requires sudo)
INSTALL_DIR="/usr/bin"
sudo mv verse "$INSTALL_DIR/"
sudo chmod +x "$INSTALL_DIR/verse"

echo "verse installed to $INSTALL_DIR. It should now be available in your PATH."
# Clean up
tmp_cleanup() {
    rm -rf "$TMP_DIR"
}
trap tmp_cleanup EXIT
