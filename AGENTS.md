# Engineering Principles for Neflo

This document outlines the engineering principles and architectural patterns followed in the Neflo repository. AI agents and developers should adhere to these when contributing to the project.

## 1. Rust Best Practices
- **Edition**: Use Rust 2021 edition.
- **Error Handling**: Use the `anyhow` crate for flexible and descriptive error management. Prefer `Result` over `unwrap()` or `expect()`.
- **Serialization**: Use `serde` and `serde_json` for all data persistence and configuration.
- **CLI**: Use `clap` with the derive feature for command-line argument parsing.
- **Linting**: Ensure code passes `cargo clippy -- -D warnings`.

## 2. Architectural Patterns
- **State Machine**: The core tracking logic in `src/tracker.rs` is implemented as a state machine. State transitions (e.g., Focus -> Idle) must be handled explicitly and should trigger immediate data persistence.
- **Centralized Logic**:
    - Statistics calculation must reside in `src/stats.rs`.
    - Time and duration formatting must reside in `src/utils.rs`.
    - Avoid duplicating logic between the TUI (`src/tui.rs`) and CLI reports (`src/report.rs`).
- **macOS Integration**: System-level idle detection is implemented in `src/system.rs` using the `CoreGraphics` framework via FFI. Avoid adding non-macOS dependencies unless they are properly gated with `#[cfg(target_os = "macos")]`.

## 3. Data Persistence
- **Storage Location**: Application data is stored in `~/.neflo/`.
- **Safety**:
    - Data is persisted to disk upon every state transition.
    - An automatic save occurs every 30 seconds.
    - A final save is performed upon application termination.
- **Format**: All persisted data is in human-readable JSON format.

## 4. TUI Development
- **Library**: Use `ratatui` and `crossterm`.
- **Event Loop**: The TUI runs in a non-blocking loop that polls for both system events (e.g., keyboard input) and internal state updates (e.g., idle time ticks).
- **Layout**: Follow the established vertical layout: Header, Summary Stats, Activity Chart, and Footer.

## 5. Testing and Verification
- **Unit Testing**: Core logic, especially the state machine and statistics engine, must be covered by unit tests.
- **Hermetic Tests**: Use the `tempfile` crate to ensure that tests do not modify the user's actual database or configuration files.
- **Determinism**: When testing time-dependent logic, allow for explicit timestamps to be passed to functions (see `Tracker::update_db`).

## 6. Dependency Management
- **Versioning**: Use major/minor versioning in `Cargo.toml` (e.g., `0.4` instead of `0.4.43`) to maintain compatibility while allowing patch updates.
- **Environment**: Ensure the project builds and passes tests on both Intel and Apple Silicon macOS environments.

---

*Note: For all tasks, agents are expected to use a deep planning mode, asking clarifying questions and verifying assumptions before proceeding with changes.*
