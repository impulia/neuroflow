# Neflo

Neflo is a lightweight macOS menu bar application that tracks your focus and idle time. It helps you understand your productivity patterns by monitoring system activity and displaying real-time statistics in a sleek tray popover.

## Features

- **Menu Bar App**: Lives in your macOS menu bar with a clean, glassmorphic popover UI built with Svelte and Tauri.
- **Activity Tracking**: Monitors system-wide keyboard and mouse activity to detect when you are actively working.
- **Smart Idle Detection**: Automatically transitions to "Idle" state after a configurable period of inactivity (default: 5 minutes).
- **Event-Driven Architecture**: The Rust backend pushes real-time updates to the frontend via Tauri events — no polling required.
- **Visual Analytics**: Weekly bar chart showing focus vs. idle time, daily goal progress ring, and motivational banners.
- **Persistence**: Stores all tracking data locally in JSON format (`~/.neflo/`).
- **Configurable**: Adjust idle threshold, daily goals, session duration, and more from the in-app settings panel.

## Documentation

Comprehensive documentation is available in the [doc/](doc/index.md) folder:

- [**Introduction**](doc/introduction.md): Overview of Neflo and its features.
- [**Setup**](doc/setup.md): System requirements and installation instructions.
- [**Usage**](doc/usage.md): Guide on how to use the menu bar app and settings.
- [**Architecture**](doc/architecture.md): Technical details about the Tauri + Svelte stack and event-driven design.
- [**Development**](doc/development.md): How to build, test, and contribute to the project.
- [**Publishing**](doc/publishing.md): Information on the release and publishing process.

## Requirements

- **Operating System**: macOS (uses CoreGraphics for system-wide idle detection).
- **Rust**: Version 1.85 or later.
- **Node.js**: Version 18 or later (for the Svelte frontend).

## Quick Start

```bash
# Install the Tauri CLI (one-time setup)
cargo install tauri-cli

# Install frontend dependencies
cd ui && npm install && cd ..

# Run in development mode (launches Tauri dev server)
cargo tauri dev
```

## Configuration

Neflo stores its data and configuration in `~/.neflo/`:

- `~/.neflo/db.json`: Recorded focus and idle intervals.
- `~/.neflo/config.json`: Persistent settings.

Example `config.json`:
```json
{
  "default_threshold_mins": 5,
  "duration": null,
  "daily_goal_hours": 4.0,
  "show_timer_in_menubar": false,
  "show_motivational_messages": true,
  "launch_at_login": false
}
```

## Project Structure

```text
src-tauri/src/
├── main.rs           # Entry point
├── lib.rs            # Tauri app builder, background thread, event emission
├── tracker.rs        # Core state machine (Focus ↔ Idle transitions)
├── commands.rs       # Tauri IPC commands + event payload builders
├── tray_manager.rs   # Menu bar tray icon and context menu
├── stats.rs          # Statistics calculation engine
├── storage.rs        # JSON persistence layer
├── models.rs         # Data structures (Interval, Database)
├── config.rs         # Configuration management
├── system.rs         # macOS CoreGraphics FFI for idle detection
└── utils.rs          # Formatting utilities

ui/src/
├── main.ts           # Svelte app bootstrap
├── App.svelte        # Root component (dashboard / settings views)
├── stores/tracker.ts # Reactive stores + Tauri event listener
└── lib/              # UI components (StatusHeader, StatsRow, WeeklyChart, etc.)
```

## Development

```bash
# Run tests
cargo test

# Check formatting and lints
cargo fmt --all -- --check
cargo clippy -- -D warnings
```

## License

MIT
