# Contributing to SearXNG MCP

Thank you for your interest in contributing to SearXNG MCP! We welcome contributions from the community.

## How to Contribute

### Reporting Bugs

If you find a bug, please open an issue with:
- A clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Your environment (OS, Rust version, etc.)

### Suggesting Features

Feature suggestions are welcome! Please open an issue with:
- A clear description of the feature
- Use cases and benefits
- Any implementation ideas

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests if applicable
5. Ensure `cargo test` passes
6. Commit with clear messages
7. Push to your fork
8. Open a Pull Request

## Development Setup

```bash
# Clone your fork
git clone https://github.com/yourusername/searxng-mcp.git
cd searxng-mcp

# Build
cargo build

# Run tests
cargo test

# Build release
cargo build --release
```

## Code Style

- Follow Rust best practices
- Use `cargo fmt` to format code
- Use `cargo clippy` to check for issues
- Add documentation comments

## Testing

- Add tests for new features
- Ensure existing tests pass
- Test with different SearXNG configurations

## Documentation

- Update README.md if adding features
- Add integration guides for new platforms
- Include examples

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
