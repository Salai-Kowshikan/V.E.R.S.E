#!/bin/bash

set -e

TAR_URL="https://raw.githubusercontent.com/Salai-Kowshikan/V.E.R.S.E/main/CLI/release/latest/verse.tar.gz"
TMP_DIR=$(mktemp -d)

curl -L "$TAR_URL" -o "$TMP_DIR/verse.tar.gz"

cd "$TMP_DIR"
tar -xzf verse.tar.gz

if [ ! -f "verse" ]; then
    echo "verse binary not found in the tarball."
    exit 1
fi

echo "verse installed to $INSTALL_DIR. Make sure $INSTALL_DIR is in your PATH."
INSTALL_DIR="/usr/bin"
sudo mv verse "$INSTALL_DIR/"
sudo chmod +x "$INSTALL_DIR/verse"

echo "verse installed to $INSTALL_DIR. It should now be available in your PATH."

tmp_cleanup() {
    rm -rf "$TMP_DIR"
}
trap tmp_cleanup EXIT
