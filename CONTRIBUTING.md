# Contributing to todo-rs

Thank you for your interest in contributing to todo-rs! 🎉

## How to Contribute

### Reporting Bugs

If you find a bug, please open an issue with:
- A clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Your Rust version (`rustc --version`)

### Suggesting Features

Feature suggestions are welcome! Please open an issue describing:
- The use case for the feature
- How it would work
- Any examples of similar features in other libraries

### Submitting Pull Requests

1. Fork the repository
2. Create a new branch (`git checkout -b feature/my-feature`)
3. Make your changes
4. Add tests for your changes
5. Ensure all tests pass (`cargo test`)
6. Run `cargo fmt` and `cargo clippy`
7. Commit your changes with a clear message
8. Push to your fork
9. Open a Pull Request

## Development Setup

```bash
# Clone the repository
git clone https://github.com/seichiki/todo-rs.git
cd todo-rs

# Run tests
cargo test

# Run the example
cargo run --example demo

# Build documentation
cargo doc --open

# Format code
cargo fmt

# Run clippy
cargo clippy
```

## Code Guidelines

- Follow Rust's standard naming conventions
- Add documentation comments for public APIs
- Write tests for new functionality
- Keep commits focused and atomic
- Write clear commit messages

## Testing

All new features should include tests. Run the test suite with:

```bash
cargo test
```

## Questions?

Feel free to open an issue for any questions about contributing!
