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

### Duplicate Logic (commands.rs ↔ tray_manager.rs)
- Three operations (pause, resume, reset today) exist in **two places**: as Tauri commands in `commands.rs` and as tray fallback functions in `tray_manager.rs`. The tray fallbacks exist so menu actions work even when the webview is not loaded. **If you change the logic in one, you must change the other.**

### IPC Type Contract
- The Rust `ConfigResponse` struct and the TypeScript `ConfigResponse` interface must always have the same fields. Two `Config` fields (`show_state_icon`, `auto_check_updates`) are **intentionally excluded** from `ConfigResponse` because they have no UI yet. When adding UI for them, update both sides.

### macOS Integration
- System-level idle detection is implemented in `src-tauri/src/system.rs` using the `CoreGraphics` framework via FFI. Avoid adding non-macOS dependencies unless properly gated with `#[cfg(target_os = "macos")]`.

### Working Directory
- **The root `Cargo.toml` is a legacy CLI artifact.** All Rust commands (`cargo test`, `cargo clippy`, `cargo fmt`) must be run from `src-tauri/`, not from the repo root.

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

## 6. Testing, Verification, and Invariant Enforcement

### Invariants File
- **[INVARIANTS.md](INVARIANTS.md)** contains every active acceptance criterion for the project. It is the single source of truth for what the system guarantees.
- **Before every push**, agents **must** verify that no invariants have been broken by their changes. This is non-negotiable.

### Mandatory Pre-Push Verification

Every change must pass the following checks before being committed/pushed:

```bash
# IMPORTANT: The root Cargo.toml is a legacy CLI crate.
# All Rust commands MUST run from src-tauri/.

# 1. Code is formatted
cd src-tauri && cargo fmt --all -- --check && cd ..

# 2. No clippy warnings
cd src-tauri && cargo clippy -- -D warnings && cd ..

# 3. All Rust unit tests pass
cd src-tauri && cargo test && cd ..

# 4. Frontend builds without errors
cd ui && npm run build && cd ..
```

### Invariant Review Process

After passing automated checks, agents must:

1. **Identify affected invariants**: Determine which invariant IDs from `INVARIANTS.md` are touched by the change (e.g., a change to `tracker.rs` likely affects SM-* and DP-* invariants).
2. **Verify each affected invariant**: Read the relevant source code and confirm the invariant still holds. If the invariant references specific values (thresholds, defaults, formats), verify those values in the code.
3. **Update INVARIANTS.md if behavior intentionally changes**: If a change deliberately alters an invariant (e.g., changing a default value), the corresponding entry in `INVARIANTS.md` **must** be updated in the same commit. Stale invariants are as dangerous as broken ones.
4. **Add new invariants for new features**: When adding a new feature, add corresponding invariant entries to `INVARIANTS.md` in the same commit.

### Unit Testing Standards
- Core logic (state machine, statistics engine, storage) must be covered by unit tests.
- **Hermetic Tests**: Use the `tempfile` crate to ensure tests do not modify the user's actual data.
- **Determinism**: When testing time-dependent logic, pass explicit timestamps (see `Tracker::update_db`).

## 7. Dependency Management

- **Rust**: Managed via `src-tauri/Cargo.toml`. Use major/minor versioning (e.g., `"2"` not `"2.1.3"`).
- **Frontend**: Managed via `ui/package.json`. Use caret ranges for semver.
- **Environment**: Ensure the project builds on both Intel and Apple Silicon macOS.

## 8. Mandatory Documentation Updates

- **Consistency**: Any change to the codebase (new features, refactoring, or bug fixes) **must** be accompanied by an update to the relevant documentation in `doc/` or `README.md`.
- **Invariants**: Any change that adds, removes, or modifies a user-facing behavior **must** update `INVARIANTS.md` in the same commit. See §6 for the full process.

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
