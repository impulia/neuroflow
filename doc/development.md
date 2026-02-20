# Development Guide

This guide is for developers who want to contribute to Neflo or build it from source.

## Project Structure

```text
src/
├── main.rs       # Entry point and CLI parsing
├── tracker.rs    # Core logic and state machine
├── tui.rs        # Terminal User Interface
├── stats.rs      # Statistics calculation
├── storage.rs    # File I/O and persistence
├── models.rs     # Data structures
├── config.rs     # Configuration management
├── system.rs     # macOS-specific FFI
├── report.rs     # CLI reporting logic
└── utils.rs      # Formatting and common utilities
```

## Building

To build the project in debug mode:
```bash
cargo build
```

To build for production:
```bash
cargo build --release
```

## Testing

Neflo has a suite of unit tests covering core logic, storage, and utility functions.

Run all tests:
```bash
cargo test
```

We use the `tempfile` crate in tests to ensure that the actual user database is never modified during testing.

## Coding Standards

- **Rust Idioms**: Follow standard Rust conventions. Use `clippy` to check for common mistakes.
- **Error Handling**: Use the `anyhow` crate for flexible error management.
- **Formatting**: Always run `cargo fmt` before committing.

## Contribution Workflow

1. Fork the repo.
2. Create a feature branch.
3. Implement your changes and add tests if applicable.
4. Run `cargo test` and `cargo clippy`.
5. Submit a Pull Request.

---

[Home](index.md) | [Previous: Architecture](architecture.md) | [Next: Publishing](publishing.md)
