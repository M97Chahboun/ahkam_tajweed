# Contributing to Tajweed Warsh Rules

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

Please be respectful and constructive in all interactions.

## Getting Started

1. Fork the repository
2. Clone your fork locally
3. Create a feature branch: `git checkout -b feature/your-feature-name`
4. Install Rust if you haven't already: https://rustup.rs/

## Development Setup

```bash
# Clone the repository
git clone https://github.com/m97chahboun/tajweed_warsh_rules.git
cd tajweed_warsh_rules

# Build the project
cargo build

# Run tests
cargo test

# Run the CLI
cargo run

# Build documentation
cargo doc --open
```

## Making Changes

### Code Style

- Follow Rust conventions (rustfmt, clippy)
- Use meaningful variable names
- Add comments for complex logic
- Update documentation comments as needed

### Running Code Quality Checks

```bash
# Format code
cargo fmt

# Check for warnings
cargo clippy

# Run tests
cargo test

# Build documentation
cargo doc --no-deps
```

## Testing

Please ensure:

1. All existing tests pass: `cargo test`
2. New code has appropriate tests
3. Tests cover both success and edge cases
4. Test names are descriptive

## Commit Messages

Use clear, descriptive commit messages:

```
Add support for X rule

- Implement detection logic for X
- Add unit tests
- Update documentation
```

## Pull Request Process

1. Ensure all tests pass: `cargo test`
2. Run `cargo fmt` to format code
3. Run `cargo clippy` to check for warnings
4. Update README.md if adding new features
5. Update CHANGELOG.md with your changes
6. Submit PR with clear description of changes

## Feature Ideas

Areas where contributions are welcome:

- **New Tajweed Rules**: Implementing additional rules or variants
- **Performance**: Optimizations for faster processing
- **Documentation**: Improving guides and examples
- **Testing**: Additional test cases and edge cases
- **CLI Enhancements**: Better user interface for the CLI tool
- **Output Formats**: JSON, XML, or other output formats
- **Internationalization**: Support for multiple languages in descriptions

## Reporting Issues

When reporting bugs, please include:

- Clear description of the issue
- Steps to reproduce
- Expected vs actual behavior
- Rust version: `rustc --version`
- OS and version

## Questions?

Feel free to open an issue or discussion for questions about contributing.

## Recognition

Contributors will be credited in the project documentation.

Thank you for making this project better!
