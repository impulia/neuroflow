use crate::config::{save_config, Config};
use crate::models::IntervalType;
use crate::stats::calculate_stats;
use crate::system::get_idle_time;
use crate::tracker::Tracker;
use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::State;

// ---------------------------------------------------------------------------
// Response types
// ---------------------------------------------------------------------------

#[derive(Serialize)]
pub struct CurrentState {
    pub state: String,
    pub elapsed_secs: i64,
    pub session_elapsed_secs: i64,
    pub idle_time: f64,
    pub paused: bool,
}

#[derive(Serialize)]
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

#[derive(Serialize)]
pub struct DayChartData {
    pub label: String,
    pub focus_secs: i64,
    pub idle_secs: i64,
    pub is_today: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigResponse {
    pub threshold_mins: u64,
    pub duration: Option<String>,
    pub daily_goal_hours: f64,
    pub show_timer_in_menubar: bool,
    pub show_motivational_messages: bool,
    pub launch_at_login: bool,
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn get_current_state(
    tracker: State<'_, Arc<Mutex<Tracker>>>,
) -> Result<CurrentState, String> {
    let guard = tracker.lock().map_err(|e| e.to_string())?;
    let now = chrono::Utc::now();

    let state_str = if guard.paused {
        "waiting".to_string()
    } else {
        match guard.last_kind_seen {
            None => "waiting".to_string(),
            Some(IntervalType::Focus) => "focus".to_string(),
            Some(IntervalType::Idle) => "idle".to_string(),
        }
    };

    let elapsed_secs = (now - guard.state_start).num_seconds();
    let session_elapsed_secs = (now - guard.run_start_time).num_seconds();
    let idle_time = get_idle_time();

    Ok(CurrentState {
        state: state_str,
        elapsed_secs,
        session_elapsed_secs,
        idle_time,
        paused: guard.paused,
    })
}

#[tauri::command]
pub fn get_stats(
    tracker: State<'_, Arc<Mutex<Tracker>>>,
    config: State<'_, Arc<Mutex<Config>>>,
) -> Result<StatsResponse, String> {
    let guard = tracker.lock().map_err(|e| e.to_string())?;
    let cfg = config.lock().map_err(|e| e.to_string())?;

    let stats = calculate_stats(&guard.db, Some(guard.run_start_time));

    // Best streak: longest consecutive focus interval today.
    let today = Local::now().date_naive();
    let mut best_streak_secs: i64 = 0;
    for interval in &guard.db.intervals {
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

    Ok(StatsResponse {
        today_focus_secs: stats.today_summary.total_focus.num_seconds(),
        today_idle_secs: stats.today_summary.total_idle.num_seconds(),
        today_interruptions: stats.today_summary.idle_count,
        session_focus_secs: stats.session_summary.total_focus.num_seconds(),
        session_idle_secs: stats.session_summary.total_idle.num_seconds(),
        best_streak_secs,
        daily_goal_secs,
        week_focus_secs: stats.week_summary.total_focus.num_seconds(),
    })
}

#[tauri::command]
pub fn get_weekly_chart_data(
    tracker: State<'_, Arc<Mutex<Tracker>>>,
) -> Result<Vec<DayChartData>, String> {
    let guard = tracker.lock().map_err(|e| e.to_string())?;
    let stats = calculate_stats(&guard.db, Some(guard.run_start_time));

    let today = Local::now().date_naive();
    // Find Monday of the current week.
    let days_from_monday = today.weekday().num_days_from_monday();
    let week_start = today - Duration::days(days_from_monday as i64);

    let day_labels = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

    let mut result = Vec::with_capacity(7);
    for offset in 0..7i64 {
        let day: NaiveDate = week_start + Duration::days(offset);
        let label = day_labels[offset as usize].to_string();
        let is_today = day == today;

        let (focus_secs, idle_secs) = if let Some(ds) = stats.daily_stats.get(&day) {
            (
                ds.total_focus.num_seconds(),
                ds.total_idle.num_seconds(),
            )
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

    Ok(result)
}

#[tauri::command]
pub fn get_config(
    config: State<'_, Arc<Mutex<Config>>>,
) -> Result<ConfigResponse, String> {
    let cfg = config.lock().map_err(|e| e.to_string())?;
    Ok(ConfigResponse {
        threshold_mins: cfg.default_threshold_mins,
        duration: cfg.duration.clone(),
        daily_goal_hours: cfg.daily_goal_hours,
        show_timer_in_menubar: cfg.show_timer_in_menubar,
        show_motivational_messages: cfg.show_motivational_messages,
        launch_at_login: cfg.launch_at_login,
    })
}

#[tauri::command]
pub fn update_config(
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

    Ok(())
}

#[tauri::command]
pub fn pause_tracking(
    tracker: State<'_, Arc<Mutex<Tracker>>>,
) -> Result<(), String> {
    let mut guard = tracker.lock().map_err(|e| e.to_string())?;
    guard.paused = true;
    Ok(())
}

#[tauri::command]
pub fn resume_tracking(
    tracker: State<'_, Arc<Mutex<Tracker>>>,
) -> Result<(), String> {
    let mut guard = tracker.lock().map_err(|e| e.to_string())?;
    guard.paused = false;
    // Reset state_start so elapsed doesn't jump on resume.
    guard.state_start = chrono::Utc::now();
    Ok(())
}

#[tauri::command]
pub fn reset_today(
    tracker: State<'_, Arc<Mutex<Tracker>>>,
) -> Result<(), String> {
    let mut guard = tracker.lock().map_err(|e| e.to_string())?;

    let today = Local::now().date_naive();
    guard.db.intervals.retain(|i| {
        let date = i.start.with_timezone(&Local).date_naive();
        date != today
    });

    guard
        .storage
        .save(&guard.db)
        .map_err(|e| e.to_string())?;

    Ok(())
}
