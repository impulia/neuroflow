# Neuroflow — Acceptance Criteria

## Instructions for agents
- **Before marking any task complete**, verify every criterion in this file still holds.
- **When product requirements change** (feature added, removed, or modified), update this file to match — add new criteria, remove obsolete ones, or amend existing ones.
- Criteria are grouped by capability. Each bullet is independently verifiable.

---

## App presence
- The app appears only in the macOS menu bar — no Dock icon, no app switcher entry.
- The menu bar icon is a brain outline when idle, a filled brain when running, and a pause circle when interrupted.
- When a session is running, the elapsed focus time (current segment) is shown next to the icon in `MM:SS` format, switching to `HH:MM:SS` once it reaches one hour.

## Session states
- The app is always in exactly one of three states: **idle**, **running**, or **interrupted**.
- From idle, the user can start a session → state becomes running.
- From running, the user can end the session → state returns to idle and the session is saved.
- From running, the user can interrupt → state becomes interrupted and the current segment is recorded.
- From interrupted, the user can resume → state becomes running and a new segment begins.
- Stopping from interrupted also saves the session.
- `start()` while already running is a no-op.
- `interrupt()` while idle or already interrupted is a no-op.
- `stop()` while idle is a no-op (no record saved).

## Focus ring (popover UI)
- The popover is 300 px wide.
- A circular progress ring shows the current segment's elapsed time as a fraction of a 50-minute goal.
- The ring is grey when idle, purple-to-cyan gradient when running, orange-to-yellow when interrupted.
- The center of the ring displays a `MM:SS` / `HH:MM:SS` counter for the current segment.
- The ring animates smoothly on state transitions.

## Stats
- A stats row shows: total session time (all segments combined) and interruption count for the active session.
- Interruption count is green when zero, orange when above zero.
- All counters reset to zero when a new session starts.

## Action buttons
- "Start Focus" button starts a session from idle; label changes to "End Session" while running; "Resume" while interrupted.
- "Interrupt" button is only enabled while running.
- Primary button colour: green (idle/interrupted), red (running).

## State badge
- A badge at the bottom of the popover shows "Focusing", "Interrupted", or "Idle" with a matching colour dot.

## Idle auto-detection
- When auto-detect is enabled (default: on), the app monitors system idle time via IOKit every 2 seconds.
- If the system has been idle for longer than the configured threshold while running, the session is automatically interrupted.
- If the system becomes active again (idle < 3 s) while interrupted, the session automatically resumes.
- Auto-detect can be toggled on/off in Settings.

## Idle threshold
- The idle threshold is configurable from 1 to 30 minutes in 1-minute steps (default: 5 minutes).
- The threshold setting is persisted across app restarts.

## Global hotkeys
- Two independent global hotkeys can be configured: Start / Stop and Interrupt.
- Hotkeys are system-wide — they fire even when the app is not in focus.
- Each hotkey requires at least one modifier key (⌘, ⌥, ⌃, or ⇧).
- Hotkeys are recorded via an in-app overlay: pressing Esc cancels recording.
- A configured hotkey is shown as a symbol string (e.g. `⌃⌥⌘F`); unset shows "Not set".
- Each hotkey has a clear (✕) button to remove it.
- Hotkeys are persisted across app restarts.

## Settings window
- Settings are accessible via the gear icon in the popover header and via the standard Settings menu.
- Settings window is 480 × 420 pt.
- Saving settings closes the window.

## Session persistence
- Completed sessions are written to `~/Library/Application Support/neuroflow/sessions.json`.
- Each record includes: id, startDate, endDate, totalFocusSeconds, interruptionCount, and the list of segments.
- Each segment includes: id, startDate, endDate, durationSeconds.
- Dates are stored in ISO 8601 format.
- A session with zero focus seconds (interrupted immediately and stopped) is still saved.
- Sessions accumulate across app restarts — existing records are never overwritten.
- Corrupt JSON files result in an empty array (graceful degradation), not a crash.

## Build and test requirements
- The project builds with zero errors on Xcode 15+ / macOS 14+.
- All 78 tests pass (75 unit tests in `neuroflowTests` + 3 UI tests in `neuroflowUITests`).
- No third-party dependencies are present.

## Code invariants (must not be broken)
- `FocusSessionManager` is the single source of truth for all session state.
- `SessionStore.append()` is called exactly once per completed session, inside `stop()`.
- `HotkeyCenter` is only configured via `FocusSessionManager.registerHotkeys()`.
- All timer callbacks dispatch to `@MainActor` via `Task { @MainActor in … }`.
- Hotkeys use Carbon modifier bitmasks, not `NSEvent.ModifierFlags`.
- `FocusSessionManager` accepts `sessionStore:` and `enableTimers:` for test injection.
- `SessionStore` has `init(fileURL:)` for test isolation — tests never touch `SessionStore.shared`.
- `SessionStore.save(_:)` has internal (not private) access for test verification.

## Test coverage summary
| Test struct | What it covers | Test count |
|-------------|---------------|------------|
| `FocusSegmentTests` | Duration calculation, ID generation, Codable, Equatable | 8 |
| `FocusSessionRecordTests` | dayKey/weekOfYear/year, Codable round-trip, multi-segment | 8 |
| `HotkeyTests` | isEmpty, displayString (all key types), Codable, Equatable | 14 |
| `SessionStateTests` | All cases exist, Equatable | 2 |
| `TimeFormattingTests` | asHMS, asMS, asAdaptiveTime, overflow | 4 |
| `SessionStoreTests` | Load empty, append, accumulate, preserve fields, corrupt file, ISO 8601 | 9 |
| `FocusSessionManagerTests` | All state transitions, toggle, multi-interrupt cycles, idempotency, persistence | 27 |
| `HotkeyCenterHelperTests` | Carbon modifier conversion from NSEvent flags | 3 |
| **Total unit tests** | | **75** |
