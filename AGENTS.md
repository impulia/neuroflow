# Neuroflow — Agent Instructions

> Read `CLAUDE.md` first. This file adds agent-specific guidance on top of it.

## Project summary
macOS 14+ menu bar focus tracker. Swift + SwiftUI, no external deps. Single binary, tray-only (`LSUIElement`). Sessions stored as JSON locally.

## File map
| File | Role | Testable? |
|------|------|-----------|
| `neuroflow/neuroflowApp.swift` | Entry point. `MenuBarExtra` + `Settings` scene. | No (app lifecycle) |
| `neuroflow/FocusModels.swift` | Value types: `FocusSegment`, `FocusSessionRecord`, `Hotkey`, `SessionState`, `Int` time extensions. | Yes — pure logic, no side effects |
| `neuroflow/FocusSessionManager.swift` | `@MainActor ObservableObject`. All session state, tick timer, idle detection, hotkey wiring. Also hosts `SessionStore`. | Yes — via `init(sessionStore:enableTimers:)` |
| `neuroflow/HotkeyCenter.swift` | Carbon `RegisterEventHotKey` wrapper. Two slots: start/stop and interrupt. | Partially — `carbonModifiers(from:)` is testable |
| `neuroflow/MenuBarView.swift` | 300 px popover: focus ring, stats row, action buttons, state badge. | No (SwiftUI view) |
| `neuroflow/SettingsView.swift` | 480×420 settings window: idle threshold slider, hotkey recorder. | No (SwiftUI view) |
| `neuroflow/ContentView.swift` | Placeholder — not used in production flow. | No |
| `neuroflow/Assets.xcassets` | App icon and accent colour only. | No |
| `neuroflowTests/neuroflowTests.swift` | 75 unit tests covering all logic. | — |
| `neuroflowUITests/` | UI automation tests (boilerplate). | — |

## Key invariants agents must preserve
1. `FocusSessionManager` is the **only** mutable state owner. Views read via `@ObservedObject`, never write to `private(set)` properties.
2. All timer callbacks dispatch back to `@MainActor` via `Task { @MainActor in … }`.
3. `sessionStore.append()` is called exactly once per completed session, inside `stop()`.
4. `HotkeyCenter.shared` is configured by `FocusSessionManager.registerHotkeys()` — do not call it from views.
5. Hotkeys are stored as Carbon modifier bitmasks (`UInt32`) + virtual key code (`UInt16`), not as `NSEvent` flags.
6. `SessionStore` has two init paths: `private init()` for the singleton, `init(fileURL:)` for tests. Do not add a third.
7. `FocusSessionManager` has two init params: `sessionStore` (default `.shared`) and `enableTimers` (default `true`). Tests must use `enableTimers: false`.
8. `save(_:)` on `SessionStore` is internal access (not private) to support test verification. Do not make it private.
9. The state machine has exactly 3 states and 4 valid transitions (see `CLAUDE.md`). Invalid transitions are no-ops, not errors.
10. All 78 tests must pass before any PR or task completion.

## Common hallucination risks — verify before using
| Risk | Correct answer |
|------|---------------|
| `SessionStore` location | Co-located in `FocusSessionManager.swift`, NOT a separate file |
| `SessionStore.save()` access | `func save()` (internal), NOT `private func save()` |
| `FocusSessionManager.init` | Takes `sessionStore:` and `enableTimers:` params |
| `Hotkey.isEmpty` | True when BOTH `keyCode == 0` AND `carbonModifiers == 0` |
| `Int.asMS()` overflow | `3600.asMS()` returns `"60:00"`, NOT an error |
| `asAdaptiveTime()` threshold | Switches at `>= 3600`, not `> 3600` |
| `interruptionCount` on double interrupt | Does NOT increment — `interrupt()` guards `state == .running` |
| `stop()` from idle | No-op, no record saved |
| `start()` while running | No-op, counters unchanged |
| `sessionStartDate` on resume | Preserved from original `start()`, NOT reset |
| `segmentStartDate` on resume | Reset to new `Date()` |
| Test framework | Swift Testing (`@Test`, `#expect`), NOT XCTest (`XCTAssert`) |
| Test imports | Must `import AppKit` for `NSEvent.ModifierFlags` |
| Carbon modifier values | `cmdKey=256`, `shiftKey=512`, `optionKey=2048`, `controlKey=4096` |

## Adding features — checklist
1. [ ] Does it require a new file? Justify. Prefer extending an existing one.
2. [ ] Does it add a dependency? Get explicit approval first.
3. [ ] Does it touch persistence? Update `FocusSessionRecord` and bump the JSON carefully (keep backwards-compat decode).
4. [ ] Does it add UI? Keep the popover <= 300 px wide and respect the existing `animation(.spring(…))` on state changes.
5. [ ] Does it change the state machine? Update tests in `FocusSessionManagerTests` and the state machine table in `CLAUDE.md`.
6. [ ] Does it add a UserDefaults key? Add it to the UserDefaults table in `CLAUDE.md`.
7. [ ] Are all 78 tests still passing?
8. [ ] Does `ACCEPTANCE.md` need updating?

## Modifying tests — rules
- Test file: `neuroflowTests/neuroflowTests.swift` (single file, all test structs)
- Each test struct maps to a source type: `FocusSegmentTests`, `FocusSessionRecordTests`, `HotkeyTests`, `SessionStateTests`, `TimeFormattingTests`, `SessionStoreTests`, `FocusSessionManagerTests`, `HotkeyCenterHelperTests`
- `@MainActor` attribute is required on `FocusSessionManagerTests` (manager is `@MainActor`)
- `SessionStoreTests` use temp directories — each test creates its own via `UUID().uuidString`
- Never test against `SessionStore.shared` — always use `init(fileURL:)` with a temp path
- `FocusSessionManagerTests` use `makeManager()` helper that returns `(FocusSessionManager, SessionStore)` tuple

## Coding rules (enforced)
- No `DispatchQueue` — use `async/await`
- No `@AppStorage` — `UserDefaults` wired in `FocusSessionManager.init`
- No force-unwrap on external data (JSON decode, file I/O)
- Prefer `guard let` / early return over nested `if let`
- Match existing naming: `camelCase` vars, `PascalCase` types, `MARK: -` section headers

## Acceptance criteria
All acceptance criteria live in `ACCEPTANCE.md`. Verify every item passes before marking work complete. Update the file whenever product requirements change.

## Build & test
- Open `neuroflow.xcodeproj` in Xcode 15+. Scheme: `neuroflow`. Run on macOS 14+ only.
- No build scripts — standard Xcode build.
- Tests: Use `RunAllTests` (78 total: 75 unit + 3 UI). All must pass.
- To run a subset: `RunSomeTests` with `targetName: "neuroflowTests"` and the struct name as identifier (e.g. `"FocusSessionManagerTests"`).

## History note
This repo previously contained a Rust TUI implementation (see commit history on `main`). The Swift rewrite lives in `neuroflow/` and `neuroflowTests/`. Ignore all `.rs` files and `Cargo.*` in git history.
