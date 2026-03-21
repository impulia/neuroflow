# CLAUDE.md — Agent Operating Manual for Neflo

This file is automatically read by Claude Code on session start. It is the
**primary entry point** for agents working in this repository.

## Quick Orientation

Neflo is a **macOS menu bar app** that tracks focus/idle time. It consists of:
- **Rust backend** (Tauri 2) in `src-tauri/`
- **Svelte 5 frontend** in `ui/`

> **The root `Cargo.toml` is a legacy CLI artifact. Ignore it.**
> All Rust work happens inside `src-tauri/`.

## Mandatory Reading

Before making any change, read these in order:
1. **This file** — you're here
2. **[AGENTS.md](AGENTS.md)** — engineering principles, architectural patterns
3. **[INVARIANTS.md](INVARIANTS.md)** — acceptance criteria (80+ invariants that must hold true)

## File Map

```
src-tauri/
  src/
    lib.rs          — App entry point, single-instance lock, background thread
    tracker.rs      — State machine (Focus ↔ Idle), interval management
    stats.rs        — Statistics aggregation (today, session, week, daily)
    commands.rs     — Tauri IPC commands + response types + emit_update()
    config.rs       — Config struct, load/save from ~/.neflo/config.json
    storage.rs      — Database persistence (atomic write to ~/.neflo/db.json)
    models.rs       — Data structures: Interval, IntervalType, Database
    tray_manager.rs — Tray icon, context menu, fallback state accessors
    system.rs       — macOS idle detection via CoreGraphics FFI
    utils.rs        — Duration formatting helper

ui/
  src/
    stores/tracker.ts  — Svelte stores + event subscription + command invocations
    App.svelte         — Root component (dashboard vs settings view)
    lib/
      StatusHeader.svelte      — State dot + label + elapsed timer
      MotivationalBanner.svelte — Context-aware encouragement messages
      StatsRow.svelte           — Focus progress ring + interruptions + streak
      WeeklyChart.svelte        — 7-bar stacked chart (Mon–Sun)
      Settings.svelte           — All settings + double-click reset
      Footer.svelte             — Weekly total + settings gear
      ProgressRing.svelte       — SVG circular progress indicator
      StatCard.svelte           — Reusable stat display card
    assets/styles.css           — CSS custom properties (design tokens)
```

## Working Directory Rules

```bash
# Rust commands — ALWAYS from src-tauri/
cd src-tauri && cargo test
cd src-tauri && cargo clippy -- -D warnings
cd src-tauri && cargo fmt --all -- --check

# Frontend commands — ALWAYS from ui/
cd ui && npm run build

# Do NOT run cargo commands from the repo root.
# The root Cargo.toml is a legacy CLI crate and will test/build the wrong thing.
```

## Pre-Push Verification (exact commands)

```bash
# Run ALL of these before committing:
cd src-tauri && cargo fmt --all -- --check && cd ..
cd src-tauri && cargo clippy -- -D warnings && cd ..
cd src-tauri && cargo test && cd ..
cd ui && npm run build && cd ..
```

## IPC Type Contract

The Rust backend and TypeScript frontend share a type contract. When modifying
IPC types, **both sides must be updated in the same commit**.

| Rust struct (commands.rs)  | TypeScript interface (tracker.ts) |
|----------------------------|-----------------------------------|
| `CurrentState`             | `StateResponse`                   |
| `StatsResponse`            | `StatsResponse`                   |
| `DayChartData`             | `DayChartData`                    |
| `ConfigResponse`           | `ConfigResponse`                  |
| `TrackerUpdate`            | `TrackerUpdate`                   |

### Intentional Config Field Gap

`ConfigResponse` intentionally omits two `Config` fields that are not yet
exposed in the UI:
- `show_state_icon` (Config has it, ConfigResponse does not)
- `auto_check_updates` (Config has it, ConfigResponse does not)

When adding UI for these fields, you must add them to `ConfigResponse` (Rust),
`ConfigResponse` (TypeScript), `build_config_response()`, and `update_config()`.

## Duplicate Logic Warning

The following logic exists in **two places** and must be kept in sync:

| Operation      | Primary (commands.rs)          | Fallback (tray_manager.rs)       |
|----------------|-------------------------------|----------------------------------|
| Pause          | `pause_tracking()`            | `pause_via_state()`              |
| Resume         | `resume_tracking()`           | `resume_via_state()`             |
| Reset Today    | `reset_today()`               | `reset_today_via_state()`        |

The tray fallbacks exist so menu actions work even when the webview is not loaded.
**If you change the logic in one, you must change the other.**

## Adding a New Feature — Checklist

1. **Plan**: Identify which files need changes (use the file map above)
2. **Write tests first** in the relevant `#[cfg(test)]` module
3. **Implement backend** changes in `src-tauri/src/`
4. **Update IPC types** if the feature adds/changes data sent to the frontend
5. **Implement frontend** changes in `ui/src/`
6. **Update INVARIANTS.md** with new acceptance criteria (same commit)
7. **Update docs** in `doc/` if user-facing behavior changes
8. **Run pre-push verification** (see above)
9. **Review affected invariant IDs** from INVARIANTS.md

## Adding a New Tauri Command — Checklist

1. Add the function in `commands.rs` with `#[tauri::command]`
2. Register it in the `invoke_handler![]` macro in `lib.rs`
3. Add the TypeScript `invoke()` wrapper in `tracker.ts`
4. If it mutates state: call `emit_update()` at the end
5. If it needs a tray fallback: add a `*_via_state()` function in `tray_manager.rs`

## Adding a New Config Field — Checklist

1. Add the field to `Config` struct in `config.rs` with `#[serde(default)]`
2. Set the default in `impl Default for Config`
3. Add to `ConfigResponse` in `commands.rs` (both struct and `build_config_response()`)
4. Add to `update_config()` to apply the new field
5. Add to `ConfigResponse` interface in `tracker.ts`
6. Add to `defaultConfig` in `tracker.ts`
7. Add UI control in `Settings.svelte`
8. Add CF-* invariant to INVARIANTS.md

## Anti-Patterns — Do NOT Do These

| Anti-Pattern | Why |
|-------------|-----|
| Run `cargo test` from repo root | Tests the legacy CLI crate, not the Tauri app |
| Add frontend polling (`setInterval` + `invoke`) | Architecture is push-based via events (EV-6) |
| Update stores directly in action functions | Stores are updated only via the event listener (FS-4) |
| Use inline colors or magic numbers in CSS | Use CSS custom properties from `styles.css` |
| Add non-macOS deps without `#[cfg]` gate | CoreGraphics FFI is macOS-only |
| Change one side of duplicated logic | Must change both commands.rs AND tray_manager.rs |
| Manually bump version or edit CHANGELOG.md | Automated by release-plz via Conventional Commits |
| Add `unwrap()` or `expect()` in library code | Use `anyhow::Result` and `?` operator |
| Import stores directly in leaf components | Leaf components receive data via props only |
| Create new files without updating the file map | Update this file's File Map section |

## Conventional Commits

PR titles **must** match: `^(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)(\(.+\))?!?: .+`

Examples: `feat: add daily goal notification`, `fix: correct week boundary calculation`

## Invariant Enforcement

After every change, identify affected invariant IDs from [INVARIANTS.md](INVARIANTS.md)
and verify each one still holds by reading the source code. If your change
intentionally alters an invariant, update INVARIANTS.md in the same commit.
See AGENTS.md §6 for the full process.
