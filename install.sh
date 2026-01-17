#!/bin/bash

# Autocommiter Universal Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/nathfavour/autocommiter.cli/main/install.sh | bash

set -e

REPO="nathfavour/autocommiter.cli"
GITHUB_URL="https://github.com/$REPO"

# Detect OS
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$ARCH" in
    x86_64) ARCH="amd64" ;;
    aarch64|arm64) ARCH="arm64" ;;
    *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

if [ "$OS" = "darwin" ]; then
    OS="darwin"
elif [ "$OS" = "linux" ]; then
    OS="linux"
else
    echo "Unsupported OS: $OS"
    exit 1
fi

BINARY_NAME="autocommiter"
ARCHIVE_EXTENSION="tar.gz"
if [ "$OS" = "windows" ]; then
    ARCHIVE_EXTENSION="zip"
fi

echo "Detected Platform: $OS/$ARCH"

# Get latest release tag
echo "Fetching release metadata..."
LATEST_TAG=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep -Po '"tag_name": "\K.*?(?=")')

if [ -z "$LATEST_TAG" ]; then
    echo "Error: Failed to fetch latest release from GitHub."
    exit 1
fi

echo "Resolved version: $LATEST_TAG"

# Download archive
# Goreleaser format: autocommiter_linux_amd64.tar.gz
DOWNLOAD_FILE="autocommiter_${OS}_${ARCH}.${ARCHIVE_EXTENSION}"
DOWNLOAD_URL="$GITHUB_URL/releases/download/$LATEST_TAG/$DOWNLOAD_FILE"

echo "Downloading $DOWNLOAD_FILE ($LATEST_TAG)..."
TMP_DIR=$(mktemp -d)
curl -fsSL "$DOWNLOAD_URL" -o "$TMP_DIR/$DOWNLOAD_FILE"

# Extract
cd "$TMP_DIR"
if [ "$ARCHIVE_EXTENSION" = "tar.gz" ]; then
    tar -xzf "$DOWNLOAD_FILE"
else
    unzip -q "$DOWNLOAD_FILE"
fi

# Install binary
INSTALL_DIR="/usr/local/bin"
if [ ! -w "$INSTALL_DIR" ]; then
    if [ -d "$HOME/.local/bin" ]; then
        INSTALL_DIR="$HOME/.local/bin"
    else
        echo "Need sudo to install to /usr/local/bin"
        INSTALL_DIR="/usr/local/bin"
        SUDO="sudo"
    fi
fi

if [ ! -d "$INSTALL_DIR" ]; then
    mkdir -p "$INSTALL_DIR"
fi

$SUDO mv autocommiter "$INSTALL_DIR/autocommiter"
$SUDO chmod +x "$INSTALL_DIR/autocommiter"

echo "Successfully installed autocommiter to $INSTALL_DIR/autocommiter"

# Cleanup
rm -rf "$TMP_DIR"

# Verify
"$INSTALL_DIR/autocommiter" --help || true
