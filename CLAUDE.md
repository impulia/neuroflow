# Neuroflow — Claude Instructions

## What this is
A macOS menu bar focus tracker. Tray-only app (no Dock icon, `LSUIElement = YES`). Tracks focus sessions, detects idle via IOKit, supports global hotkeys via Carbon.

## Stack
- Swift 5.10+, SwiftUI, macOS 14+
- No external dependencies — pure Apple frameworks only
- Storage: `SessionStore` writes JSON to `~/Library/Application Support/neuroflow/sessions.json`
- Hotkeys: Carbon Event API (`HotkeyCenter.swift`)
- Idle detection: IOKit `IOHIDSystem / HIDIdleTime`

## Architecture
MVVM. One source of truth: `FocusSessionManager` (`@MainActor`, `ObservableObject`).

```
neuroflowApp          — entry point, MenuBarExtra + Settings scene
FocusSessionManager   — all session state, timers, idle detection, hotkey wiring
SessionStore          — JSON persistence (singleton)
HotkeyCenter          — Carbon global hotkey registration (singleton)
FocusModels           — pure value types: FocusSegment, FocusSessionRecord, Hotkey, SessionState
MenuBarView           — popover UI (300 px wide)
SettingsView          — settings window (480×420)
```

## Session lifecycle
`idle → running → interrupted → running → … → idle`
- `start()` / `stop()` / `interrupt()` / `toggleStartStop()` on `FocusSessionManager`
- Idle auto-detection checks every 2 s; threshold configurable (default 5 min)
- On stop, a `FocusSessionRecord` (with segments) is appended to `SessionStore`

## Principles — follow strictly
- **YAGNI**: only implement what is currently needed
- **KISS**: simplest correct solution
- **Minimum lines**: no unnecessary abstractions, helpers, or wrappers
- Do **not** add docstrings, comments, or type annotations to code you didn't change
- Do **not** add error handling for impossible cases
- Do **not** create utilities for one-off operations

## Conventions
- `@MainActor` on all UI-touching classes
- `private(set)` for published state that views should not mutate directly
- Carbon modifier flags (not `NSEvent.ModifierFlags`) for hotkey storage
- Time formatting via `Int.asAdaptiveTime()` — shows HMS only when ≥ 1 hour
- `SessionState` is an enum: `.idle`, `.running`, `.interrupted`

## What NOT to do
- No notifications (not planned)
- No cloud sync or network calls
- No third-party dependencies without explicit user approval
- Do not use `@AppStorage` — settings are persisted manually via `UserDefaults` in `FocusSessionManager`
- Do not use `GCD` / `DispatchQueue` — use `async/await` + `Task { @MainActor in … }`

## Testing
Unit-test pure logic in `FocusModels` and `SessionStore`. Do not mock `FocusSessionManager` internals — test behaviour through its public API.

Before concluding any task, verify all criteria in `ACCEPTANCE.md` still hold. Update `ACCEPTANCE.md` if requirements change.
