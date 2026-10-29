#!/usr/bin/env bash
set -e

REPO="ale-01cu/codexco"
INSTALL_DIR="$HOME/.codexco/bin"
BIN_NAME="codexco"

echo "Installing codexco..."

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)
    TARGET="x86_64-unknown-linux-gnu"
    ;;
  Darwin)
    if [ "$ARCH" = "arm64" ]; then
      TARGET="aarch64-apple-darwin"
    else
      TARGET="x86_64-apple-darwin"
    fi
    ;;
  *)
    echo "Unsupported OS: $OS"
    exit 1
    ;;
esac

ARCHIVE_NAME="codexco-${TARGET}.tar.gz"
DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${ARCHIVE_NAME}"

mkdir -p "$INSTALL_DIR"

TMP_DIR="$(mktemp -d)"
echo "Downloading ${DOWNLOAD_URL}..."
curl -fsSL "$DOWNLOAD_URL" -o "${TMP_DIR}/${ARCHIVE_NAME}"

echo "Extracting to ${INSTALL_DIR}..."
tar -xzf "${TMP_DIR}/${ARCHIVE_NAME}" -C "$INSTALL_DIR"
chmod +x "${INSTALL_DIR}/${BIN_NAME}"
rm -rf "$TMP_DIR"

# Add to PATH if missing
SHELL_CONFIG=""
if [ -n "$BASH_VERSION" ]; then
  SHELL_CONFIG="$HOME/.bashrc"
elif [ -n "$ZSH_VERSION" ]; then
  SHELL_CONFIG="$HOME/.zshrc"
fi

if [ -n "$SHELL_CONFIG" ] && [ -f "$SHELL_CONFIG" ]; then
  if ! grep -q "$INSTALL_DIR" "$SHELL_CONFIG"; then
    echo "export PATH=\"\$PATH:${INSTALL_DIR}\"" >> "$SHELL_CONFIG"
    echo "Added ${INSTALL_DIR} to ${SHELL_CONFIG}"
  fi
fi

echo ""
echo "✓ codexco successfully installed to ${INSTALL_DIR}/${BIN_NAME}!"
echo "Restart your terminal or run: export PATH=\"\$PATH:${INSTALL_DIR}\""
