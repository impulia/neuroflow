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
