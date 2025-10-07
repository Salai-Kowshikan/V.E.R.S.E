#!/bin/bash

set -euo pipefail

# --- Preflight checks -------------------------------------------------------
missing=0

require_cmd() {
	if ! command -v "$1" >/dev/null 2>&1; then
		echo "Error: required command '$1' not found." >&2
		missing=1
	fi
}

# Core tools needed to build a Rust binary
require_cmd cargo
require_cmd rustc

# A C linker is needed by Rust crates during linking
if ! command -v cc >/dev/null 2>&1 && ! command -v gcc >/dev/null 2>&1 && ! command -v clang >/dev/null 2>&1; then
	echo "Error: no C compiler/linker found (cc/gcc/clang)." >&2
	missing=1
fi

if [[ $missing -ne 0 ]]; then
	echo >&2
	echo "To install a C toolchain (example commands):" >&2
	if [[ -f /etc/os-release ]]; then
		. /etc/os-release
		case "${ID:-}" in
			ubuntu|debian)
				echo "  sudo apt update && sudo apt install -y build-essential" >&2 ;;
			linuxmint|pop|neon|kali)
				echo "  sudo apt update && sudo apt install -y build-essential" >&2 ;;
			fedora)
				echo "  sudo dnf groupinstall -y 'Development Tools'" >&2 ;;
			rhel|centos|rocky|almalinux)
				echo "  sudo yum groupinstall -y 'Development Tools'" >&2 ;;
			arch|manjaro)
				echo "  sudo pacman -S --needed base-devel" >&2 ;;
			opensuse*|sles)
				echo "  sudo zypper install -t pattern devel_C_C++" >&2 ;;
			*)
				echo "  Install gcc/clang and make via your distro's package manager." >&2 ;;
		esac
	else
		echo "  Install gcc/clang and make via your distro's package manager." >&2
	fi
	exit 1
fi

# --- Build the binary in release mode --------------------------------------
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
