# Neflo — Acceptance Criteria & Invariants

Every item below is a **currently active** acceptance criterion. Before merging any
change, agents and developers **must** verify that none of these invariants have
been broken. The test plan in [AGENTS.md](AGENTS.md) explains *how* to verify
them.

> **Rule of thumb:** if a line in this file becomes false after your change, you
> have introduced a regression. Fix it before pushing.

---

## 1. State Machine (`src-tauri/src/tracker.rs`)

| ID | Invariant |
|----|-----------|
| SM-1 | Exactly two states exist: `Focus` and `Idle`. There are no intermediate states. |
| SM-2 | `idle_time >= threshold_secs` → state is `Idle`; `idle_time < threshold_secs` → state is `Focus`. |
| SM-3 | `threshold_secs` equals `threshold_mins * 60` exactly (integer multiplication, no rounding). |
| SM-4 | A state transition is recorded when `current_kind != last_kind_seen`. |
| SM-5 | On every transition, `state_start` is set to `now` and `last_kind_seen` is updated. |
| SM-6 | On every transition, `storage.save()` is called immediately. |
| SM-7 | On Focus→Idle, the idle start is backdated: `idle_start = now - idle_time`. |
| SM-8 | If `idle_start <= last_interval.start`, the entire last interval is converted to Idle (no split). |
| SM-9 | If `idle_start > last_interval.start`, the last interval is split at `idle_start`. The first part remains Focus ending at `idle_start`; the second part is a new Idle interval from `idle_start` to `now`. |
| SM-10 | On Idle→Focus, the idle interval is closed at `now` and a new Focus interval is created starting at `now`. |
| SM-11 | A gap > 10 seconds between `now` and `last_interval.end` creates a new interval (does not extend the previous one). |
| SM-12 | Same-kind ticks with no gap extend the last interval's `end` to `now`. |
| SM-13 | Zero-duration or negative-duration intervals are pruned after every `update_db` call. |

## 2. Data Persistence (`src-tauri/src/storage.rs`, `tracker.rs`)

| ID | Invariant |
|----|-----------|
| DP-1 | Writes are atomic: data is written to `db.tmp`, then renamed to `db.json`. |
| DP-2 | A periodic save occurs every 30 seconds regardless of state transitions. |
| DP-3 | A save occurs on every state transition (SM-6). |
| DP-4 | A save occurs when `should_stop()` fires and `session_ended_saved` is false. |
| DP-5 | `prune_old_data()` removes intervals whose `end <= (now - 30 days)`. It runs on tracker init and every 30-second save. |
| DP-6 | Loading a nonexistent `db.json` returns an empty `Database` (no error). |
| DP-7 | All persisted data is human-readable pretty-printed JSON. |
| DP-8 | Application data lives in `~/.neflo/` (`db.json`, `config.json`, `neflo.lock`). |

## 3. Single-Instance Lock (`src-tauri/src/lib.rs`)

| ID | Invariant |
|----|-----------|
| SI-1 | An exclusive write lock is acquired on `~/.neflo/neflo.lock` at startup. |
| SI-2 | If the lock cannot be acquired, the process prints "Neflo is already running." to stderr and exits with code 1. |
| SI-3 | The lock is held for the entire lifetime of the process. |

## 4. Configuration (`src-tauri/src/config.rs`)

| ID | Invariant |
|----|-----------|
| CF-1 | Default `default_threshold_mins` = 5. |
| CF-2 | Default `duration` = `None`. |
| CF-3 | Default `daily_goal_hours` = 4.0. |
| CF-4 | Default `show_timer_in_menubar` = false. |
| CF-5 | Default `show_state_icon` = true. |
| CF-6 | Default `launch_at_login` = false. |
| CF-7 | Default `auto_check_updates` = true. |
| CF-8 | Default `show_motivational_messages` = true. |
| CF-9 | If `config.json` does not exist, it is created with all defaults on first load. |
| CF-10 | Unknown fields in `config.json` are silently ignored (forward compatible via serde defaults). |
| CF-11 | `save_config()` overwrites the entire file (no partial merge). |

## 5. Event-Driven Architecture (`src-tauri/src/lib.rs`, `commands.rs`)

| ID | Invariant |
|----|-----------|
| EV-1 | The background thread emits a `tracker-update` Tauri event every ~1 second. |
| EV-2 | The event is emitted when paused (UI must stay in sync with paused state). |
| EV-3 | The event is emitted when the session has ended (UI must show ended state). |
| EV-4 | The `tracker-update` payload contains four fields: `state` (`CurrentState`), `stats` (`StatsResponse`), `weekly` (`Vec<DayChartData>`), `config` (`ConfigResponse`). |
| EV-5 | Mutation commands (`pause_tracking`, `resume_tracking`, `reset_today`, `update_config`) emit `tracker-update` immediately after mutating state, so the UI updates without waiting for the next tick. |
| EV-6 | The frontend **does not** poll the backend. All periodic data comes from event subscription. |

## 6. Tauri Commands (`src-tauri/src/commands.rs`)

### Query Commands

| ID | Invariant |
|----|-----------|
| QC-1 | `get_current_state()` returns `state` as one of: `"waiting"`, `"focus"`, `"idle"`. If paused, always `"waiting"`. |
| QC-2 | `elapsed_secs` = `(Utc::now() - tracker.state_start).num_seconds()`. |
| QC-3 | `session_elapsed_secs` = `(Utc::now() - tracker.run_start_time).num_seconds()`. |
| QC-4 | `get_stats()` `today_interruptions` equals the count of Idle intervals from today (local timezone). |
| QC-5 | `best_streak_secs` is the duration of the longest single Focus interval from today. |
| QC-6 | `daily_goal_secs` = `config.daily_goal_hours * 3600` (truncated to integer). |
| QC-7 | `get_weekly_chart_data()` always returns exactly 7 elements, in order: Mon, Tue, Wed, Thu, Fri, Sat, Sun. |
| QC-8 | Exactly one element in the weekly data has `is_today = true`; the rest are false. |
| QC-9 | Days with no recorded intervals have `focus_secs: 0, idle_secs: 0`. |

### Mutation Commands

| ID | Invariant |
|----|-----------|
| MC-1 | `pause_tracking()` sets `tracker.paused = true`. |
| MC-2 | `resume_tracking()` sets `tracker.paused = false` and resets `tracker.state_start` to `Utc::now()` (elapsed resets to 0). |
| MC-3 | `reset_today()` removes **only** intervals whose start date (in local timezone) equals today. Past days' data is preserved. |
| MC-4 | `reset_today()` saves the database to disk immediately after filtering. |
| MC-5 | `update_config()` applies all fields from the request atomically to in-memory config, persists to disk, and updates `tracker.threshold_secs` from the new `threshold_mins`. |

## 7. Statistics Engine (`src-tauri/src/stats.rs`)

| ID | Invariant |
|----|-----------|
| ST-1 | `session_summary` includes only intervals where `interval.start >= run_start_time`. |
| ST-2 | `today_summary` includes only intervals from the current local date. |
| ST-3 | `week_summary` includes intervals from Monday through Sunday of the current local week. |
| ST-4 | Week start is Monday, computed via `weekday().num_days_from_monday()`. |
| ST-5 | Intervals with negative duration (`end < start`) are skipped. |
| ST-6 | `daily_stats` is a `BTreeMap<NaiveDate, DayStats>` covering every day that has at least one interval. |

## 8. Tray Manager (`src-tauri/src/tray_manager.rs`)

| ID | Invariant |
|----|-----------|
| TM-1 | Left-clicking the tray icon toggles the webview window visibility. |
| TM-2 | When shown, the window is positioned at `TrayCenter` and receives focus. |
| TM-3 | The context menu contains exactly: Pause Tracking, Resume Tracking, (separator), Reset Today, (separator), Quit Neflo. |
| TM-4 | "Quit Neflo" calls `app.exit(0)`. |
| TM-5 | Tray menu actions (pause, resume, reset) work even when the webview is not loaded, by directly accessing tracker state. |

## 9. macOS System Integration (`src-tauri/src/system.rs`)

| ID | Invariant |
|----|-----------|
| SY-1 | On macOS, `get_idle_time()` returns the result of `CGEventSourceSecondsSinceLastEventType` (non-negative `f64`). |
| SY-2 | On non-macOS platforms, `get_idle_time()` returns `0.0` (always Focus). |

## 10. Frontend Store (`ui/src/stores/tracker.ts`)

| ID | Invariant |
|----|-----------|
| FS-1 | `startListening()` is idempotent: calling it multiple times creates only one event subscription. |
| FS-2 | `stopListening()` is safe to call at any time, including when no subscription exists. |
| FS-3 | On each `tracker-update` event, all four stores (`currentState`, `stats`, `weeklyData`, `config`) are updated. |
| FS-4 | Action functions (`pauseTracking`, `resumeTracking`, `resetToday`, `updateConfig`) do **not** update stores directly; they rely on the backend to emit an event. |
| FS-5 | Default `currentState.state` is `"waiting"`, `paused` is `false`. |
| FS-6 | Default `stats.daily_goal_secs` is `14400` (4 hours). |
| FS-7 | Default `config.threshold_mins` is `5`, `daily_goal_hours` is `4`. |

## 11. UI Components (`ui/src/lib/`)

### StatusHeader

| ID | Invariant |
|----|-----------|
| UI-1 | `effectiveState` is `"paused"` when `paused = true`, otherwise equals `state`. |
| UI-2 | State labels: focus→"In Flow", idle→"Idle", paused→"Paused", ended→"Session Ended", default→"Waiting". |
| UI-3 | Dot colors: focus→green (animated), idle→amber (animated), paused→white (static), waiting→faint white (static). |
| UI-4 | Elapsed time is formatted as `Xh Ym Zs`, `Ym Zs`, or `Zs` (largest non-zero unit first). |

### MotivationalBanner

| ID | Invariant |
|----|-----------|
| UI-5 | Banner is hidden if `show = false` OR the computed message is empty. |
| UI-6 | Focus messages: <10min→"Getting into the zone...", 10-29min→"Deep work in progress — X minutes strong", 30-49min→"Incredible focus session!", ≥50min→"Outstanding! Over X minutes of deep work". |
| UI-7 | Idle message: "Ready to dive back in?". Ended message: "Great session! Time to rest." |
| UI-8 | Waiting/paused states produce an empty message (banner hidden). |

### StatsRow & ProgressRing

| ID | Invariant |
|----|-----------|
| UI-9 | Goal progress is clamped to `[0, 1]`: `Math.min(1, today_focus_secs / daily_goal_secs)`. |
| UI-10 | Three stat cards are displayed: Focus progress (with ring), Interruptions, Best Streak. |
| UI-11 | Interruptions card color: green if count is 0, amber if > 0. |
| UI-12 | ProgressRing `clampedValue` is `Math.min(1, Math.max(0, value))`. Dashoffset = `circumference * (1 - clampedValue)`. |

### WeeklyChart

| ID | Invariant |
|----|-----------|
| UI-13 | If `data` is empty, 7 zero-valued dummy bars (Mon–Sun) are rendered. |
| UI-14 | `maxTotal` is at least 1 (prevents division by zero). |
| UI-15 | Today's bar has a green `drop-shadow` glow and 100% focus opacity. |
| UI-16 | Bars are stacked: idle (amber) on top, focus (green) on bottom. |
| UI-17 | Hover tooltip shows day label, focus time, and idle time. |

### Settings

| ID | Invariant |
|----|-----------|
| UI-18 | Reset requires two clicks within 3 seconds. First click shows "Confirm?" with a pulsing animation; second click executes the reset. |
| UI-19 | If no second click within 3 seconds, the reset button reverts to its default state. |
| UI-20 | Every settings control calls `onSave` immediately on change (no explicit save button). |
| UI-21 | Idle threshold range: 1–30 minutes. Daily goal range: 0.5–12 hours (step 0.5). |

### Footer

| ID | Invariant |
|----|-----------|
| UI-22 | Left side shows "This week: **Xh Ym** focused". |
| UI-23 | Right side shows a gear icon that calls `onSettingsClick`. |

### App Layout

| ID | Invariant |
|----|-----------|
| UI-24 | `showSettings = false` renders the dashboard view (StatusHeader, MotivationalBanner, StatsRow, WeeklyChart, Footer). |
| UI-25 | `showSettings = true` renders the Settings view. |
| UI-26 | `onMount` calls `startListening()`; `onDestroy` calls `stopListening()`. |

## 12. Duration Formatting (`src-tauri/src/utils.rs`)

| ID | Invariant |
|----|-----------|
| FMT-1 | `format_duration(0)` returns `"0s"`. |
| FMT-2 | Output format: `Xd Xh Xm Xs`, omitting zero components except when all are zero. |
| FMT-3 | Seconds component is included if all larger units are zero, even when seconds is 0. |

---

## Verification Checklist

Before pushing any change, run:

```bash
# 1. Code is formatted
cargo fmt --all -- --check

# 2. No clippy warnings
cargo clippy -- -D warnings

# 3. All Rust unit tests pass
cargo test

# 4. Frontend builds without errors
cd ui && npm run build && cd ..
```

Then review the invariant IDs touched by your change and confirm they still hold
by reading the relevant source code.
