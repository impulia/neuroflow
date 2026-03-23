# Neuroflow — Claude Instructions

## What this is
A macOS menu bar focus tracker. Tray-only app (no Dock icon, `LSUIElement = YES`). Tracks focus sessions with segments, detects idle via IOKit, supports global hotkeys via Carbon. No main window — interaction is through a `MenuBarExtra` popover and a `Settings` window.

## Stack
- Swift 5.10+, SwiftUI, macOS 14+
- No external dependencies — pure Apple frameworks only (Foundation, AppKit, SwiftUI, IOKit, Carbon)
- Storage: `SessionStore` writes JSON to `~/Library/Application Support/neuroflow/sessions.json`
- Hotkeys: Carbon Event API (`HotkeyCenter.swift`)
- Idle detection: IOKit `IOHIDSystem / HIDIdleTime`
- Testing: Swift Testing framework (`import Testing`), not XCTest

## Architecture
MVVM. One source of truth: `FocusSessionManager` (`@MainActor`, `ObservableObject`).

```
neuroflowApp          — entry point, MenuBarExtra + Settings scene
FocusSessionManager   — all session state, timers, idle detection, hotkey wiring
SessionStore          — JSON persistence (co-located in FocusSessionManager.swift)
HotkeyCenter          — Carbon global hotkey registration (singleton)
FocusModels           — pure value types: FocusSegment, FocusSessionRecord, Hotkey, SessionState
MenuBarView           — popover UI (300 px wide)
SettingsView          — settings window (480×420)
ContentView           — placeholder, not used in production flow
```

## Session lifecycle
`idle → running → interrupted → running → … → idle`
- `start()` / `stop()` / `interrupt()` / `toggleStartStop()` on `FocusSessionManager`
- Idle auto-detection checks every 2 s; threshold configurable (default 5 min)
- On stop, a `FocusSessionRecord` (with segments) is appended to `SessionStore`
- Each uninterrupted focus period is a `FocusSegment` with its own `startDate` / `endDate`
- `finalizeCurrentSegment()` is called on both `interrupt()` and `stop()` — only if `currentFocusSeconds > 0`

## State machine — exact transitions
```
idle        → start()        → running
running     → stop()         → idle        (saves record)
running     → interrupt()    → interrupted (increments interruptionCount, finalizes segment)
interrupted → start()        → running     (new segment begins, sessionStartDate preserved)
interrupted → stop()         → idle        (saves record)
idle        → stop()         → no-op
idle        → interrupt()    → no-op
running     → start()        → no-op
interrupted → interrupt()    → no-op
```

## Key types and their exact fields

### FocusSegment (Codable, Equatable, Identifiable)
- `id: UUID` (auto-generated if omitted)
- `startDate: Date`
- `endDate: Date`
- `durationSeconds: Int` (computed in init as `Int(endDate.timeIntervalSince(startDate))`)

### FocusSessionRecord (Codable, Identifiable, Equatable)
- `id: UUID` (auto-generated if omitted)
- `startDate: Date`
- `endDate: Date`
- `totalFocusSeconds: Int`
- `interruptionCount: Int`
- `segments: [FocusSegment]`
- Computed: `dayKey: String` (yyyy-MM-dd), `weekOfYear: Int`, `year: Int`

### Hotkey (Codable, Equatable)
- `keyCode: UInt16` — Carbon virtual key code (e.g. `kVK_ANSI_F = 3`)
- `carbonModifiers: UInt32` — Carbon modifier bitmask (`cmdKey = 256`, `shiftKey = 512`, `optionKey = 2048`, `controlKey = 4096`)
- `static let empty` — keyCode 0, carbonModifiers 0
- `isEmpty: Bool` — true only when both keyCode AND carbonModifiers are 0
- `displayString() -> String` — renders as `⌃⌥⌘F` or `Not set`

### SessionState (enum, Equatable)
- `.idle`, `.running`, `.interrupted`

### Int time extensions
- `asHMS() -> String` — always `HH:MM:SS` (e.g. `01:02:03`)
- `asMS() -> String` — always `MM:SS` (e.g. `62:03` — can overflow 59)
- `asAdaptiveTime() -> String` — `MM:SS` below 3600, `HH:MM:SS` at 3600+

## FocusSessionManager — published properties
| Property | Type | Access | Notes |
|----------|------|--------|-------|
| `state` | `SessionState` | `private(set)` | Current state machine state |
| `currentFocusSeconds` | `Int` | `private(set)` | Seconds in current segment |
| `totalSessionSeconds` | `Int` | `private(set)` | Total seconds across all segments |
| `interruptionCount` | `Int` | `private(set)` | Number of interruptions in current session |
| `autoDetectIdle` | `Bool` | read-write | Persisted via UserDefaults key `"autoDetectIdle"` |
| `idleThresholdMinutes` | `Int` | read-write | Persisted via UserDefaults key `"idleThresholdMinutes"` |
| `startStopHotkey` | `Hotkey` | read-write | Persisted as JSON via UserDefaults key `"startStopHotkey"` |
| `interruptHotkey` | `Hotkey` | read-write | Persisted as JSON via UserDefaults key `"interruptHotkey"` |

## FocusSessionManager — computed properties
| Property | Type | Logic |
|----------|------|-------|
| `idleThresholdSeconds` | `Int` | `idleThresholdMinutes * 60` |
| `isRunning` | `Bool` | `state == .running` |
| `isInterrupted` | `Bool` | `state == .interrupted` |
| `isActive` | `Bool` | `state != .idle` |

## FocusSessionManager — init parameters
- `sessionStore: SessionStore = .shared` — injectable for testing
- `enableTimers: Bool = true` — set `false` in tests to avoid tick/idle timers

## SessionStore
- `static let shared` — singleton for production, writes to `~/Library/Application Support/neuroflow/sessions.json`
- `init(fileURL: URL)` — package-internal init for test isolation
- `append(_ record:)` — loads all, appends, saves
- `loadAll() -> [FocusSessionRecord]` — returns `[]` if file missing or corrupt
- `save(_ records:)` — overwrites file with ISO 8601 dates, pretty-printed sorted-keys JSON

## HotkeyCenter
- Singleton via `HotkeyCenter.shared`
- `onStartStop: (() -> Void)?` — callback for start/stop hotkey
- `onInterrupt: (() -> Void)?` — callback for interrupt hotkey
- `register(startStop:interrupt:)` — unregisters old, registers new
- `unregisterAll()` — removes both hotkey registrations
- `static carbonModifiers(from: NSEvent.ModifierFlags) -> UInt32` — converts AppKit flags to Carbon
- Uses Carbon `EventHotKeyID` with signatures `"nfss"` (id 1) and `"nfit"` (id 2)

## Carbon modifier constants (used throughout)
- `cmdKey = 256`
- `shiftKey = 512`
- `optionKey = 2048`
- `controlKey = 4096`

## UserDefaults keys (complete list)
| Key | Type | Default | Written by |
|-----|------|---------|------------|
| `"autoDetectIdle"` | `Bool` | `true` | `FocusSessionManager.autoDetectIdle.didSet` |
| `"idleThresholdMinutes"` | `Int` | `5` | `FocusSessionManager.idleThresholdMinutes.didSet` |
| `"startStopHotkey"` | `Data` (JSON) | nil | `FocusSessionManager.persistHotkey()` |
| `"interruptHotkey"` | `Data` (JSON) | nil | `FocusSessionManager.persistHotkey()` |

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
- Time formatting via `Int.asAdaptiveTime()` — shows HMS only when >= 1 hour
- `SessionState` is an enum: `.idle`, `.running`, `.interrupted`
- `MARK: -` section headers in all files
- `camelCase` for properties/methods, `PascalCase` for types

## What NOT to do
- No notifications (not planned)
- No cloud sync or network calls
- No third-party dependencies without explicit user approval
- Do not use `@AppStorage` — settings are persisted manually via `UserDefaults` in `FocusSessionManager`
- Do not use `GCD` / `DispatchQueue` — use `async/await` + `Task { @MainActor in … }`
- Do not use Combine publishers — the app uses `@Published` + SwiftUI observation only
- Do not call `HotkeyCenter` from views — only `FocusSessionManager.registerHotkeys()` does this
- Do not modify `SessionStore.shared` singleton path — use `init(fileURL:)` only in tests

## Testing
- Framework: Swift Testing (`import Testing`, `@Test`, `#expect`)
- Test target: `neuroflowTests`
- Import `AppKit` in test file (needed for `NSEvent.ModifierFlags`)
- Test pure logic in `FocusModels` and `SessionStore`
- Test `FocusSessionManager` behaviour through its public API using `enableTimers: false`
- Test `SessionStore` using `init(fileURL:)` with temp directories
- Do not mock `FocusSessionManager` internals
- 75 unit tests currently cover: FocusSegment, FocusSessionRecord, Hotkey, SessionState, Int time extensions, SessionStore, FocusSessionManager state machine, HotkeyCenter helpers
- Run tests: `RunAllTests` or `RunSomeTests` with target `neuroflowTests`

## Before concluding any task
1. Build succeeds with zero errors
2. All 78 tests pass (75 unit + 3 UI)
3. Verify all criteria in `ACCEPTANCE.md` still hold
4. Update `ACCEPTANCE.md` if requirements change
