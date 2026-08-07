#!/bin/bash
# SearXNG MCP Server Installation Script

set -e

echo "=== SearXNG MCP Server Installer ==="
echo

# Find the binary
BINARY_PATH=""
if [ -f "target/release/searxng-mcp" ]; then
    BINARY_PATH="$(pwd)/target/release/searxng-mcp"
elif [ -f "$HOME/.cargo/target/release/searxng-mcp" ]; then
    BINARY_PATH="$HOME/.cargo/target/release/searxng-mcp"
else
    echo "Error: Binary not found. Please build first with: cargo build --release"
    exit 1
fi

echo "Found binary at: $BINARY_PATH"

# Install location
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
mkdir -p "$INSTALL_DIR"

# Copy binary
cp "$BINARY_PATH" "$INSTALL_DIR/searxng-mcp"
chmod +x "$INSTALL_DIR/searxng-mcp"

echo "Installed to: $INSTALL_DIR/searxng-mcp"

# Check if install dir is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo
    echo "WARNING: $INSTALL_DIR is not in your PATH!"
    echo "Add the following to your shell profile (.bashrc, .zshrc, etc.):"
    echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
fi

echo
echo "=== Installation Complete ==="
echo
echo "Next steps:"
echo "1. Make sure you have a SearXNG instance running (default: http://localhost:8888)"
echo "2. Configure your MCP client to use: $INSTALL_DIR/searxng-mcp"
echo
echo "Claude Desktop config example:"
cat <<EOF
{
  "mcpServers": {
    "searxng": {
      "command": "$INSTALL_DIR/searxng-mcp",
      "env": {
        "SEARXNG_URL": "http://localhost:8888"
      }
    }
  }
}
EOF
