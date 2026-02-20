# Architecture

Neflo is built with Rust and follows a modular design focused on reliability and performance.

## Core Components

### 1. Tracker State Machine (`src/tracker.rs`)
The `Tracker` struct acts as a state machine. It processes idle time updates (ticks) and determines transitions between `Focus` and `Idle` states.
- If idle time exceeds the threshold, the state becomes `Idle`.
- If idle time is below the threshold, the state is `Focus`.
- Transitions are recorded as `Interval` objects in the database.

### 2. macOS Integration (`src/system.rs`)
Neflo uses the macOS `CoreGraphics` framework via FFI (Foreign Function Interface) to determine the time since the last user input event (keyboard or mouse).
- Function: `CGEventSourceSecondsSinceLastEventType`
- This ensures accurate tracking without needing high-level permissions or accessibility access in most cases.

### 3. Terminal User Interface (`src/tui.rs`)
The TUI is built using the `ratatui` and `crossterm` crates. It uses a non-blocking event loop to:
- Render the dashboard at a consistent frame rate.
- Listen for keyboard input.
- Poll the system for idle time updates.

### 4. Persistence Layer (`src/storage.rs`)
Data is stored in a JSON file (`db.json`). To ensure data safety:
- Data is saved upon every state transition.
- Data is periodically saved every 30 seconds.
- A final save is performed when the application exits gracefully.

### 5. Statistics Engine (`src/stats.rs`)
Calculations for daily and weekly summaries are centralized. This ensures consistency between the TUI and the CLI reports.

## Data Model

- **Interval**: Represents a continuous period of either Focus or Idle time, defined by a `start` time, `end` time, and `kind`.
- **Database**: A simple collection of `Interval` objects.

---

[Home](index.md) | [Previous: Usage](usage.md) | [Next: Development](development.md)
