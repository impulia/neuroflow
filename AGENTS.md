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

## 7. Mandatory Documentation Updates
- **Consistency**: Any change to the codebase (new features, refactoring, or bug fixes) **must** be accompanied by an update to the relevant documentation in the `doc/` folder or the root `README.md`.

## 8. Core Engineering Principles
- **TDD (Test-Driven Development)**: Write tests before or alongside feature implementation to ensure correctness and prevent regressions.
- **SOLID**: Follow SOLID principles to ensure the software is understandable, flexible, and maintainable.
- **YAGNI (You Aren't Gonna Need It)**: Do not implement functionality until it is actually needed. Keep the scope focused on the requirements.
- **KISS (Keep It Simple, Stupid)**: Favor simple, readable solutions over complex ones. Avoid over-engineering.

## 9. Versioning and Releases
- **Conventional Commits**: All Pull Request titles must follow the [Conventional Commits](https://www.conventionalcommits.org/) specification (e.g., `feat: add new feature`, `fix: resolve bug`). This is enforced via CI and is used to automatically determine the next version and generate changelogs.
- **Automated Releases**: Neflo uses an automated release system (`release-plz`). Every merge to the `main` branch triggers an automatic version bump (if applicable), a new Git tag, a GitHub Release with compiled macOS binaries, and an update to the `CHANGELOG.md`.
- **Hands-off Process**: Do not manually update the version in `Cargo.toml` or update the `CHANGELOG.md` unless specifically instructed. The automated system handles these during the release process.

---

*Note: For all tasks, agents are expected to use a deep planning mode, asking clarifying questions and verifying assumptions before proceeding with changes.*
