# Changelog

All notable changes to this project will be documented in this file.

## [0.3.0] - Unreleased

### Added
- Tauri 2 menu bar (tray) application replacing the CLI + TUI.
- Svelte 5 frontend with glassmorphic popover UI.
- Event-driven architecture: backend pushes `tracker-update` events to the frontend instead of polling.
- Dashboard components: StatusHeader, MotivationalBanner, StatsRow (with ProgressRing), WeeklyChart, Footer.
- In-app Settings panel with idle threshold, daily goal, session duration, motivational messages, and launch-at-login controls.
- Tray context menu (Pause, Resume, Reset Today, Quit).
- Single-instance lock file (`~/.neflo/neflo.lock`).

### Changed
- Migrated from CLI (`clap`) + TUI (`ratatui`/`crossterm`) to Tauri 2 + Svelte 5 architecture.
- Replaced frontend polling (4 IPC calls/second) with push-based Tauri event system (single event/second).
- Restructured project: Rust code in `src-tauri/src/`, frontend in `ui/src/`.

### Removed
- CLI commands (`start`, `report`, `self-update`).
- Terminal User Interface (`ratatui`/`crossterm`).
- `clap`, `ctrlc`, `ratatui`, `crossterm`, and `self_update` dependencies.

## [0.2.0] - 2026-02-22

### Added
- Universal macOS binary support (aarch64 and x86_64).
- `self-update` command to easily update the CLI from GitHub releases.
- Application screenshot in `README.md` and documentation.
- Enhanced TUI dashboard with improved layout and statistics.
- Automatic duration formatting for better readability.
