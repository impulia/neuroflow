# Architecture

Neflo is built with **Rust** (Tauri backend) and **Svelte** (TypeScript frontend), following a modular, event-driven design focused on low resource usage and responsiveness.

## High-Level Overview

```
┌──────────────────────────────────────────────────┐
│                  macOS Menu Bar                   │
│                  (Tray Icon)                      │
└──────────────┬───────────────────────────────────┘
               │ left-click toggles
┌──────────────▼───────────────────────────────────┐
│              Svelte Frontend (WebView)            │
│  ┌─────────┐ ┌──────────┐ ┌───────────────────┐  │
│  │ Stores  │ │Components│ │  Event Listener   │  │
│  └────▲────┘ └──────────┘ └────────▲──────────┘  │
│       │                            │              │
│       └────── updates from ────────┘              │
└──────────────────────────────────────────────────┘
                       ▲
                       │ `tracker-update` event (every 1s)
                       │ + immediate events on mutations
┌──────────────────────┴───────────────────────────┐
│               Rust Backend (Tauri)                │
│  ┌───────────┐ ┌──────────┐ ┌────────────────┐   │
│  │ Tracker   │ │ Commands │ │ Tray Manager   │   │
│  │ (thread)  │ │ (IPC)    │ │ (menu/clicks)  │   │
│  └─────┬─────┘ └──────────┘ └────────────────┘   │
│        │                                          │
│  ┌─────▼─────┐ ┌──────────┐ ┌────────────────┐   │
│  │ State     │ │ Storage  │ │ System (FFI)   │   │
│  │ Machine   │ │ (JSON)   │ │ CoreGraphics   │   │
│  └───────────┘ └──────────┘ └────────────────┘   │
└──────────────────────────────────────────────────┘
```

## Event-Driven Data Flow

Neflo uses a **push-based (event-driven)** architecture rather than polling:

1. A **background Rust thread** runs every second, calling `tracker.tick()` to sample the system idle time and update the interval database.
2. After each tick, the thread builds a full `TrackerUpdate` payload (current state, stats, weekly chart data, and config) and **emits** it as a Tauri `tracker-update` event.
3. The **Svelte frontend** subscribes to this event via `listen()` from `@tauri-apps/api/event`. When an event arrives, it updates the reactive Svelte stores, which automatically re-render the UI.
4. **Mutation commands** (pause, resume, reset, update config) are still invoked from the frontend via Tauri's `invoke()` IPC. After each mutation, the backend also emits a `tracker-update` event so the UI reflects the change immediately — without waiting for the next tick.

This design eliminates the overhead of 4 parallel IPC round-trips per second (the old polling approach) and replaces it with a single event push.

## Core Components

### 1. Tracker State Machine (`src-tauri/src/tracker.rs`)

The `Tracker` struct acts as a state machine. It processes idle-time updates (ticks) and determines transitions between `Focus` and `Idle` states.
- If idle time exceeds the configured threshold, the state becomes `Idle`.
- If idle time is below the threshold, the state is `Focus`.
- Transitions are recorded as `Interval` objects in the database.
- On Focus→Idle transitions, the idle start is back-dated to produce accurate intervals.

### 2. macOS Integration (`src-tauri/src/system.rs`)

Neflo uses the macOS `CoreGraphics` framework via FFI to determine the time since the last user input event (keyboard or mouse).
- Function: `CGEventSourceSecondsSinceLastEventType`
- This ensures accurate tracking without needing accessibility permissions.

### 3. Tauri Commands (`src-tauri/src/commands.rs`)

IPC commands exposed to the frontend:
- **Query commands**: `get_current_state`, `get_stats`, `get_weekly_chart_data`, `get_config` — used for initial data fetch if needed.
- **Mutation commands**: `update_config`, `pause_tracking`, `resume_tracking`, `reset_today` — each emits a `tracker-update` event after mutating state.
- **Helper functions**: `build_tracker_update()` and `emit_update()` construct and push the event payload.

### 4. Tray Manager (`src-tauri/src/tray_manager.rs`)

Handles the macOS menu bar integration:
- Left-click toggles the webview popover (positioned near the tray icon via `tauri-plugin-positioner`).
- Right-click opens a context menu (Pause, Resume, Reset Today, Quit).

### 5. Svelte Frontend (`ui/src/`)

- **Stores** (`stores/tracker.ts`): Reactive Svelte writable stores for state, stats, weekly data, and config. Subscribes to `tracker-update` events via `startListening()`.
- **Components** (`lib/`): StatusHeader, MotivationalBanner, StatsRow (with ProgressRing), WeeklyChart, Footer, Settings.
- **Styling**: Glassmorphic design with CSS custom properties, backdrop-filter blur, and smooth transitions.

### 6. Persistence Layer (`src-tauri/src/storage.rs`)

Data is stored in JSON format in `~/.neflo/`:
- **Atomic Saves**: Written to a temp file then renamed to prevent corruption.
- **File Locking**: Advisory lock (`neflo.lock`) prevents multiple instances.
- **Auto-Pruning**: Records older than 30 days are removed automatically.
- **Save Triggers**: On state transitions, every 30 seconds, and on session end.

### 7. Statistics Engine (`src-tauri/src/stats.rs`)

Centralized calculations for daily, weekly, and session summaries. Used by both the IPC commands and the event emission helpers.

## Data Model

- **Interval**: A continuous period of Focus or Idle time, with `start`, `end`, and `kind`.
- **Database**: A collection of `Interval` objects, serialized to `db.json`.
- **TrackerUpdate**: The event payload containing `CurrentState`, `StatsResponse`, `Vec<DayChartData>`, and `ConfigResponse`.

---

[Home](index.md) | [Previous: Usage](usage.md) | [Next: Development](development.md)
