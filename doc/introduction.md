# Introduction to Neflo

Neflo is a lightweight, open-source macOS menu bar application that tracks your productivity and focus. By monitoring system-wide activity through the CoreGraphics framework, Neflo provides real-time insights into your "flow" states and identifies periods of inactivity — all from a sleek tray popover.

## Why Neflo?

In today's digital environment, interruptions are common. Neflo helps you:
- **Quantify Focus**: See exactly how much time you spend actively working, with a daily goal progress ring.
- **Identify Idle Patterns**: Understand when and why you're stepping away from your machine.
- **Improve Productivity**: Use data-driven insights from the weekly chart and streak tracking to optimize your work schedule.

## Key Features

- **Menu Bar Integration**: Lives in your macOS tray. Click to open the popover; click again to dismiss.
- **Real-Time Updates**: The Rust backend pushes state changes to the Svelte frontend via Tauri events every second — no polling overhead.
- **Visual Dashboard**: Status header with elapsed time, stats cards (focus progress, interruptions, best streak), weekly bar chart, and motivational banners.
- **In-App Settings**: Configure idle threshold, daily goal, session duration, motivational messages, and launch-at-login from a dedicated settings panel.
- **Privacy-First**: All data is stored locally on your machine in a simple JSON format (`~/.neflo/`).
- **Smart Detection**: Automatically distinguishes between "Focus" and "Idle" based on keyboard and mouse activity using macOS CoreGraphics.

---

[Home](index.md) | [Next: Setup](setup.md)
