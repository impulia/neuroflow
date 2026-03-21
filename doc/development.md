# Development Guide

This guide is for developers who want to contribute to Neflo or build it from source.

## Project Structure

```text
src-tauri/src/
├── main.rs           # Binary entry point
├── lib.rs            # Tauri app builder, background thread, event emission
├── tracker.rs        # Core state machine and tick logic
├── commands.rs       # Tauri IPC commands + event payload builders
├── tray_manager.rs   # Menu bar tray icon and context menu
├── stats.rs          # Statistics calculation engine
├── storage.rs        # JSON file I/O and persistence
├── models.rs         # Data structures (Interval, Database)
├── config.rs         # Configuration management
├── system.rs         # macOS CoreGraphics FFI for idle detection
└── utils.rs          # Formatting and common utilities

ui/src/
├── main.ts           # Svelte app bootstrap
├── App.svelte        # Root component (dashboard / settings routing)
├── stores/tracker.ts # Reactive stores + Tauri event listener
├── lib/              # UI components
│   ├── StatusHeader.svelte
│   ├── MotivationalBanner.svelte
│   ├── ProgressRing.svelte
│   ├── StatCard.svelte
│   ├── StatsRow.svelte
│   ├── WeeklyChart.svelte
│   ├── Footer.svelte
│   └── Settings.svelte
└── assets/
    └── styles.css    # Global styles (glassmorphic theme, CSS variables)
```

## Prerequisites

- **Rust** 1.85+ with `cargo`
- **Node.js** 18+ with `npm`
- **Tauri CLI**: `cargo install tauri-cli`
- **macOS** (required for CoreGraphics idle detection)

## Building

### Development Mode

```bash
cd ui && npm install && cd ..
cargo tauri dev
```

This starts the Vite dev server with hot-reload and launches the Tauri app.

### Production Build

```bash
cd ui && npm install && cd ..
cargo tauri build
```

The built `.app` bundle is in `src-tauri/target/release/bundle/macos/`.

## Testing

Neflo has unit tests covering the core state machine, statistics engine, and storage layer.

```bash
cargo test
```

Tests use the `tempfile` crate to ensure the user's actual database is never modified.

## Coding Standards

- **Rust**: Follow standard Rust conventions. Run `cargo clippy -- -D warnings` and `cargo fmt` before committing.
- **TypeScript/Svelte**: Follow the existing component patterns. Use TypeScript interfaces for all data contracts.
- **Error Handling**: Use the `anyhow` crate in Rust. Prefer `Result` over `unwrap()`.

## Contribution Workflow

1. Fork the repository.
2. Create a feature branch.
3. Implement your changes and add tests if applicable.
4. Run `cargo test` and `cargo clippy`.
5. Submit a Pull Request. **Ensure the PR title follows [Conventional Commits](https://www.conventionalcommits.org/) format** (e.g., `feat: add new feature`).

---

[Home](index.md) | [Previous: Architecture](architecture.md) | [Next: Publishing](publishing.md)
