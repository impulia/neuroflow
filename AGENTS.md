# Engineering Principles for Neflo

This document outlines the engineering principles and architectural patterns followed in the Neflo repository. AI agents and developers should adhere to these when contributing to the project.

## 1. Technology Stack

- **Backend**: Rust with Tauri 2 for the native macOS menu bar app shell.
- **Frontend**: Svelte 5 with TypeScript, bundled by Vite.
- **Styling**: CSS custom properties with a glassmorphic theme (backdrop-filter, blur).
- **Build**: `cargo tauri dev` for development, `cargo tauri build` for production.

## 2. Rust Best Practices

- **Edition**: Use Rust 2021 edition.
- **Error Handling**: Use the `anyhow` crate for flexible and descriptive error management. Prefer `Result` over `unwrap()` or `expect()`.
- **Serialization**: Use `serde` and `serde_json` for all data persistence, IPC payloads, and configuration.
- **Linting**: Ensure code passes `cargo clippy -- -D warnings`.

## 3. Architectural Patterns

### Event-Driven Data Flow
- The Rust backend **pushes** updates to the Svelte frontend via Tauri events (`tracker-update`). The frontend **must not** poll the backend.
- A background thread in `src-tauri/src/lib.rs` ticks the tracker every second and emits a `TrackerUpdate` event containing the full application state.
- Mutation commands (`pause_tracking`, `resume_tracking`, `reset_today`, `update_config`) also emit the event immediately after state changes so the UI updates without waiting for the next tick.
- The frontend subscribes via `listen()` from `@tauri-apps/api/event` in `ui/src/stores/tracker.ts`.

### State Machine
- The core tracking logic in `src-tauri/src/tracker.rs` is implemented as a state machine. State transitions (Focus ↔ Idle) must be handled explicitly and should trigger immediate data persistence.

### Centralized Logic
- Statistics calculation resides in `src-tauri/src/stats.rs`.
- Event payload construction resides in `src-tauri/src/commands.rs` (`build_*` helper functions and `emit_update()`).
- Avoid duplicating calculation logic between commands and the background thread — always use the shared helpers.

### macOS Integration
- System-level idle detection is implemented in `src-tauri/src/system.rs` using the `CoreGraphics` framework via FFI. Avoid adding non-macOS dependencies unless properly gated with `#[cfg(target_os = "macos")]`.

## 4. Frontend Patterns

- **Reactive Stores**: Use Svelte writable stores in `ui/src/stores/tracker.ts`. The event listener updates these stores; components subscribe reactively.
- **Component Architecture**: Small, focused Svelte components in `ui/src/lib/`. Each component receives data via props — no direct store access in leaf components.
- **Type Safety**: All data contracts between Rust and TypeScript must be defined as TypeScript interfaces in `tracker.ts` that mirror the Rust `#[derive(Serialize)]` structs.
- **Styling**: Use CSS custom properties defined in `ui/src/assets/styles.css`. No inline colors or magic numbers.

## 5. Data Persistence

- **Storage Location**: Application data is stored in `~/.neflo/`.
- **Safety**:
  - Data is persisted to disk upon every state transition.
  - An automatic save occurs every 30 seconds.
  - Data is saved when the session ends.
- **Format**: All persisted data is in human-readable JSON format.
- **Single Instance**: An advisory file lock (`neflo.lock`) prevents multiple instances.

## 6. Testing and Verification

- **Unit Testing**: Core logic (state machine, statistics engine) must be covered by unit tests.
- **Hermetic Tests**: Use the `tempfile` crate to ensure that tests do not modify the user's actual database or configuration files.
- **Determinism**: When testing time-dependent logic, allow for explicit timestamps to be passed to functions (see `Tracker::update_db`).

## 7. Dependency Management

- **Rust**: Managed via `src-tauri/Cargo.toml`. Use major/minor versioning (e.g., `"2"` not `"2.1.3"`).
- **Frontend**: Managed via `ui/package.json`. Use caret ranges for semver.
- **Environment**: Ensure the project builds on both Intel and Apple Silicon macOS.

## 8. Mandatory Documentation Updates

- **Consistency**: Any change to the codebase (new features, refactoring, or bug fixes) **must** be accompanied by an update to the relevant documentation in `doc/` or `README.md`.

## 9. Core Engineering Principles

- **TDD (Test-Driven Development)**: Write tests before or alongside feature implementation.
- **SOLID**: Follow SOLID principles for maintainable, flexible code.
- **YAGNI**: Do not implement functionality until it is actually needed.
- **KISS**: Favor simple, readable solutions over complex ones.

## 10. Versioning and Releases

- **Conventional Commits**: All Pull Request titles must follow the [Conventional Commits](https://www.conventionalcommits.org/) specification. This is enforced via CI.
- **Automated Releases**: Every merge to `main` triggers automatic version bump, changelog update, Git tag, and GitHub Release with macOS binaries.
- **Hands-off Process**: Do not manually update the version in `Cargo.toml` or `CHANGELOG.md` unless specifically instructed.

---

*Note: For all tasks, agents are expected to use a deep planning mode, asking clarifying questions and verifying assumptions before proceeding with changes.*
