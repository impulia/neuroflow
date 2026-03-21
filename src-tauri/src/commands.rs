use crate::config::{save_config, Config};
use crate::models::IntervalType;
use crate::stats::calculate_stats;
use crate::system::get_idle_time;
use crate::tracker::Tracker;
use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{Emitter, State};

// ---------------------------------------------------------------------------
// Response types
// ---------------------------------------------------------------------------

#[derive(Serialize, Clone)]
pub struct CurrentState {
    pub state: String,
    pub elapsed_secs: i64,
    pub session_elapsed_secs: i64,
    pub idle_time: f64,
    pub paused: bool,
}

#[derive(Serialize, Clone)]
pub struct StatsResponse {
    pub today_focus_secs: i64,
    pub today_idle_secs: i64,
    pub today_interruptions: u32,
    pub session_focus_secs: i64,
    pub session_idle_secs: i64,
    pub best_streak_secs: i64,
    pub daily_goal_secs: i64,
    pub week_focus_secs: i64,
}

#[derive(Serialize, Clone)]
pub struct DayChartData {
    pub label: String,
    pub focus_secs: i64,
    pub idle_secs: i64,
    pub is_today: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigResponse {
    pub threshold_mins: u64,
    pub duration: Option<String>,
    pub daily_goal_hours: f64,
    pub show_timer_in_menubar: bool,
    pub show_motivational_messages: bool,
    pub launch_at_login: bool,
}

/// Bundled payload emitted as the `tracker-update` event every tick.
#[derive(Serialize, Clone)]
pub struct TrackerUpdate {
    pub state: CurrentState,
    pub stats: StatsResponse,
    pub weekly: Vec<DayChartData>,
    pub config: ConfigResponse,
}

// ---------------------------------------------------------------------------
// Helpers – build payloads from shared state (used by both commands & events)
// ---------------------------------------------------------------------------

pub fn build_current_state(tracker: &Tracker) -> CurrentState {
    let now = chrono::Utc::now();
    let state_str = if tracker.paused {
        "waiting".to_string()
    } else {
        match tracker.last_kind_seen {
            None => "waiting".to_string(),
            Some(IntervalType::Focus) => "focus".to_string(),
            Some(IntervalType::Idle) => "idle".to_string(),
        }
    };
    let elapsed_secs = (now - tracker.state_start).num_seconds();
    let session_elapsed_secs = (now - tracker.run_start_time).num_seconds();
    let idle_time = get_idle_time();

    CurrentState {
        state: state_str,
        elapsed_secs,
        session_elapsed_secs,
        idle_time,
        paused: tracker.paused,
    }
}

pub fn build_stats(tracker: &Tracker, cfg: &Config) -> StatsResponse {
    let stats = calculate_stats(&tracker.db, Some(tracker.run_start_time));

    let today = Local::now().date_naive();
    let mut best_streak_secs: i64 = 0;
    for interval in &tracker.db.intervals {
        if interval.kind != IntervalType::Focus {
            continue;
        }
        let date = interval.start.with_timezone(&Local).date_naive();
        if date != today {
            continue;
        }
        let dur = (interval.end - interval.start).num_seconds();
        if dur > best_streak_secs {
            best_streak_secs = dur;
        }
    }

    let daily_goal_secs = (cfg.daily_goal_hours * 3600.0) as i64;

    StatsResponse {
        today_focus_secs: stats.today_summary.total_focus.num_seconds(),
        today_idle_secs: stats.today_summary.total_idle.num_seconds(),
        today_interruptions: stats.today_summary.idle_count,
        session_focus_secs: stats.session_summary.total_focus.num_seconds(),
        session_idle_secs: stats.session_summary.total_idle.num_seconds(),
        best_streak_secs,
        daily_goal_secs,
        week_focus_secs: stats.week_summary.total_focus.num_seconds(),
    }
}

pub fn build_weekly(tracker: &Tracker) -> Vec<DayChartData> {
    let stats = calculate_stats(&tracker.db, Some(tracker.run_start_time));
    let today = Local::now().date_naive();
    let days_from_monday = today.weekday().num_days_from_monday();
    let week_start = today - Duration::days(days_from_monday as i64);
    let day_labels = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

    let mut result = Vec::with_capacity(7);
    for offset in 0..7i64 {
        let day: NaiveDate = week_start + Duration::days(offset);
        let label = day_labels[offset as usize].to_string();
        let is_today = day == today;
        let (focus_secs, idle_secs) = if let Some(ds) = stats.daily_stats.get(&day) {
            (ds.total_focus.num_seconds(), ds.total_idle.num_seconds())
        } else {
            (0, 0)
        };
        result.push(DayChartData {
            label,
            focus_secs,
            idle_secs,
            is_today,
        });
    }
    result
}

pub fn build_config_response(cfg: &Config) -> ConfigResponse {
    ConfigResponse {
        threshold_mins: cfg.default_threshold_mins,
        duration: cfg.duration.clone(),
        daily_goal_hours: cfg.daily_goal_hours,
        show_timer_in_menubar: cfg.show_timer_in_menubar,
        show_motivational_messages: cfg.show_motivational_messages,
        launch_at_login: cfg.launch_at_login,
    }
}

pub fn build_tracker_update(tracker: &Tracker, cfg: &Config) -> TrackerUpdate {
    TrackerUpdate {
        state: build_current_state(tracker),
        stats: build_stats(tracker, cfg),
        weekly: build_weekly(tracker),
        config: build_config_response(cfg),
    }
}

/// Emit the full tracker update to the frontend.
pub fn emit_update(app: &tauri::AppHandle, tracker: &Tracker, cfg: &Config) {
    let payload = build_tracker_update(tracker, cfg);
    let _ = app.emit("tracker-update", payload);
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn get_current_state(tracker: State<'_, Arc<Mutex<Tracker>>>) -> Result<CurrentState, String> {
    let guard = tracker.lock().map_err(|e| e.to_string())?;
    Ok(build_current_state(&guard))
}

#[tauri::command]
pub fn get_stats(
    tracker: State<'_, Arc<Mutex<Tracker>>>,
    config: State<'_, Arc<Mutex<Config>>>,
) -> Result<StatsResponse, String> {
    let guard = tracker.lock().map_err(|e| e.to_string())?;
    let cfg = config.lock().map_err(|e| e.to_string())?;
    Ok(build_stats(&guard, &cfg))
}

#[tauri::command]
pub fn get_weekly_chart_data(
    tracker: State<'_, Arc<Mutex<Tracker>>>,
) -> Result<Vec<DayChartData>, String> {
    let guard = tracker.lock().map_err(|e| e.to_string())?;
    Ok(build_weekly(&guard))
}

#[tauri::command]
pub fn get_config(config: State<'_, Arc<Mutex<Config>>>) -> Result<ConfigResponse, String> {
    let cfg = config.lock().map_err(|e| e.to_string())?;
    Ok(build_config_response(&cfg))
}

#[tauri::command]
pub fn update_config(
    app: tauri::AppHandle,
    new_cfg: ConfigResponse,
    tracker: State<'_, Arc<Mutex<Tracker>>>,
    config: State<'_, Arc<Mutex<Config>>>,
) -> Result<(), String> {
    let mut cfg = config.lock().map_err(|e| e.to_string())?;
    cfg.default_threshold_mins = new_cfg.threshold_mins;
    cfg.duration = new_cfg.duration;
    cfg.daily_goal_hours = new_cfg.daily_goal_hours;
    cfg.show_timer_in_menubar = new_cfg.show_timer_in_menubar;
    cfg.show_motivational_messages = new_cfg.show_motivational_messages;
    cfg.launch_at_login = new_cfg.launch_at_login;

    save_config(&cfg).map_err(|e| e.to_string())?;

    // Update tracker threshold.
    let mut t = tracker.lock().map_err(|e| e.to_string())?;
    t.threshold_secs = (cfg.default_threshold_mins * 60) as f64;

    emit_update(&app, &t, &cfg);
    Ok(())
}

#[tauri::command]
pub fn pause_tracking(
    app: tauri::AppHandle,
    tracker: State<'_, Arc<Mutex<Tracker>>>,
    config: State<'_, Arc<Mutex<Config>>>,
) -> Result<(), String> {
    let mut guard = tracker.lock().map_err(|e| e.to_string())?;
    guard.paused = true;
    let cfg = config.lock().map_err(|e| e.to_string())?;
    emit_update(&app, &guard, &cfg);
    Ok(())
}

#[tauri::command]
pub fn resume_tracking(
    app: tauri::AppHandle,
    tracker: State<'_, Arc<Mutex<Tracker>>>,
    config: State<'_, Arc<Mutex<Config>>>,
) -> Result<(), String> {
    let mut guard = tracker.lock().map_err(|e| e.to_string())?;
    guard.paused = false;
    guard.state_start = chrono::Utc::now();
    let cfg = config.lock().map_err(|e| e.to_string())?;
    emit_update(&app, &guard, &cfg);
    Ok(())
}

#[tauri::command]
pub fn reset_today(
    app: tauri::AppHandle,
    tracker: State<'_, Arc<Mutex<Tracker>>>,
    config: State<'_, Arc<Mutex<Config>>>,
) -> Result<(), String> {
    let mut guard = tracker.lock().map_err(|e| e.to_string())?;

    let today = Local::now().date_naive();
    guard.db.intervals.retain(|i| {
        let date = i.start.with_timezone(&Local).date_naive();
        date != today
    });

    guard.storage.save(&guard.db).map_err(|e| e.to_string())?;

    let cfg = config.lock().map_err(|e| e.to_string())?;
    emit_update(&app, &guard, &cfg);
    Ok(())
}
