# Usage Guide

Neflo runs as a macOS menu bar (tray) application. Once launched, it sits in your menu bar and tracks your focus automatically.

## The Menu Bar Icon

After launching Neflo, a small indicator (●) appears in your macOS menu bar.

- **Left-click** the icon to toggle the dashboard popover.
- **Right-click** (or secondary click) to open the context menu with options: Pause Tracking, Resume Tracking, Reset Today, and Quit Neflo.

## The Dashboard

The dashboard popover displays your real-time tracking data in a compact, glassmorphic UI.

### Layout

- **Status Header**: Shows your current state (IN FLOW, IDLE, or WAITING) with a colored indicator and elapsed time in the current state.
- **Motivational Banner**: Context-aware messages that encourage you during focus and comfort you during idle periods. Can be toggled off in settings.
- **Stats Row**: Three cards showing:
  - **Focus Progress**: A progress ring showing today's focus time relative to your daily goal.
  - **Interruptions**: Number of idle-to-focus transitions today (green when zero, amber otherwise).
  - **Best Streak**: Your longest continuous focus session today.
- **Weekly Chart**: A stacked bar chart (Monday–Sunday) showing focus (green) and idle (amber) time for each day. Today's bar has a subtle glow. Hover any bar for exact values.
- **Footer**: Weekly focus summary and a gear icon to open settings.

## Settings Panel

Click the gear icon in the footer to open settings. Changes take effect immediately.

### Tracking
- **Idle Threshold**: How many minutes of inactivity before Neflo considers you idle (1–30 minutes, default: 5).
- **Session Duration**: Optional session limit (e.g., `4h`, `30m`). Leave empty for unlimited tracking.
- **Timer in Menu Bar**: Show elapsed focus time next to the tray icon.

### Goals
- **Daily Goal**: Target focus hours per day (1–12 hours, default: 4). Drives the progress ring.
- **Motivational Messages**: Toggle encouraging banners in the dashboard.

### System
- **Launch at Login**: Start Neflo automatically when you log in.

### Data
- **Reset Today**: Clear all tracking data for the current day. Requires confirmation (double-click within 3 seconds).

## Data Storage

Neflo stores its data and configuration locally:
- `~/.neflo/db.json`: The database of recorded focus/idle intervals (auto-pruned after 30 days).
- `~/.neflo/config.json`: Persistent settings.
- `~/.neflo/neflo.lock`: Advisory lock preventing multiple instances.

---

[Home](index.md) | [Previous: Setup](setup.md) | [Next: Architecture](architecture.md)
