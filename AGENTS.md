# Neuroflow — Agent Instructions

> Read CLAUDE.md first. This file adds agent-specific guidance on top of it.

## Project summary
macOS 14+ menu bar focus tracker. Swift + SwiftUI, no external deps. Single binary, tray-only (`LSUIElement`). Sessions stored as JSON locally.

## File map
| File | Role |
|------|------|
| `neuroflow/neuroflowApp.swift` | Entry point. `MenuBarExtra` + `Settings` scene. |
| `neuroflow/FocusModels.swift` | Value types: `FocusSegment`, `FocusSessionRecord`, `Hotkey`, `SessionState`, `Int` time extensions. |
| `neuroflow/FocusSessionManager.swift` | `@MainActor ObservableObject`. All session state, tick timer, idle detection, hotkey wiring. Also hosts `SessionStore`. |
| `neuroflow/HotkeyCenter.swift` | Carbon `RegisterEventHotKey` wrapper. Two slots: start/stop and interrupt. |
| `neuroflow/MenuBarView.swift` | 300 px popover: focus ring, stats row, action buttons, state badge. |
| `neuroflow/SettingsView.swift` | 480×420 settings window: idle threshold slider, hotkey recorder. |
| `neuroflow/ContentView.swift` | Placeholder — not used in production flow. |
| `neuroflow/Assets.xcassets` | App icon and accent colour only. |
| `neuroflowTests/` | Unit tests for pure logic. |
| `neuroflowUITests/` | UI automation tests. |

## Key invariants agents must preserve
1. `FocusSessionManager` is the **only** mutable state owner. Views read, never write.
2. All timer callbacks dispatch back to `@MainActor` via `Task { @MainActor in … }`.
3. `SessionStore.shared.append()` is called exactly once per completed session, inside `stop()`.
4. `HotkeyCenter.shared` is configured by `FocusSessionManager.registerHotkeys()` — do not call it from views.
5. Hotkeys are stored as Carbon modifier bitmasks (`UInt32`) + virtual key code (`UInt16`), not as `NSEvent` flags.

## Adding features — checklist
- [ ] Does it require a new file? Justify. Prefer extending an existing one.
- [ ] Does it add a dependency? Get explicit approval first.
- [ ] Does it touch persistence? Update `FocusSessionRecord` and bump the JSON carefully (keep backwards-compat decode).
- [ ] Does it add UI? Keep the popover ≤ 300 px wide and respect the existing `animation(.spring(…))` on state changes.

## Coding rules (enforced)
- No `DispatchQueue` — use `async/await`
- No `@AppStorage` — `UserDefaults` wired in `FocusSessionManager.init`
- No force-unwrap on external data (JSON decode, file I/O)
- Prefer `guard let` / early return over nested `if let`
- Match existing naming: `camelCase` vars, `PascalCase` types, `MARK: -` section headers

## Acceptance criteria
All acceptance criteria live in `ACCEPTANCE.md`. Verify every item passes before marking work complete. Update the file whenever product requirements change.

## Build & run
Open `neuroflow.xcodeproj` in Xcode 15+. Scheme: `neuroflow`. Run on macOS 14+ only.
No build scripts — standard Xcode build.

## History note
This repo previously contained a Rust TUI implementation (see commit history on `main`). The Swift rewrite lives in `neuroflow/` and `neuroflowTests/`. Ignore all `.rs` files and `Cargo.*` in git history.
