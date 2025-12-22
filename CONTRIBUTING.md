# Contributing to sentry-cli

Thank you for your interest in contributing to sentry-cli!

## Getting Started

### Prerequisites

- Rust 1.85+ (uses edition 2024)
- A Sentry account for integration testing

### Development Setup

```bash
# Clone the repository
git clone https://github.com/d6e/sentry-cli.git
cd sentry-cli

# Build the project
cargo build

# Run tests
cargo test

# Run with verbose output
cargo run -- -v issues list
```

## Making Changes

### Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes
4. Ensure tests pass (`cargo test`)
5. Ensure code is formatted (`cargo fmt`)
6. Ensure clippy passes (`cargo clippy`)
7. Commit your changes with a descriptive message
8. Push to your fork and open a pull request

### Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and address any warnings
- Write tests for new functionality
- Keep commits focused and atomic

### Testing

```bash
# Run all tests
cargo test

# Run a specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

## Reporting Issues

When reporting issues, please include:

- Your operating system and version
- Rust version (`rustc --version`)
- Steps to reproduce the issue
- Expected vs actual behavior
- Any relevant error messages

## Questions?

Feel free to open an issue for questions or discussion.
