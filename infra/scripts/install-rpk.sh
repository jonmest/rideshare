#!/usr/bin/env bash
set -e

OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
  Darwin) PLATFORM=darwin ;;
  Linux) PLATFORM=linux ;;
  *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

case "$ARCH" in
  x86_64|amd64) ARCH=amd64 ;;
  arm64|aarch64) ARCH=arm64 ;;
  *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

URL="https://github.com/redpanda-data/redpanda/releases/latest/download/rpk-${PLATFORM}-${ARCH}.zip"

mkdir -p "$HOME/.local/bin"
curl -L "$URL" -o /tmp/rpk.zip
unzip -o /tmp/rpk.zip -d "$HOME/.local/bin"
chmod +x "$HOME/.local/bin/rpk"

"$HOME/.local/bin/rpk" version