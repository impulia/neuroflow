use crate::models::{Database, IntervalType};
use chrono::{Datelike, Duration, Local, NaiveDate};
use std::collections::BTreeMap;

#[derive(Default, Clone, Debug)]
pub struct DayStats {
    pub total_focus: Duration,
    pub total_idle: Duration,
    pub focus_sessions: u32,
    pub idle_sessions: u32,
}

pub struct Stats {
    pub daily_stats: BTreeMap<NaiveDate, DayStats>,
    pub today: NaiveDate,
    pub week_start: NaiveDate,
}

pub fn calculate_stats(db: &Database) -> Stats {
    let now_local = Local::now();
    let today = now_local.date_naive();

    // Find the start of the current week (Monday)
    let days_from_monday = now_local.weekday().num_days_from_monday();
    let week_start = today - Duration::days(days_from_monday as i64);

    let mut daily_stats: BTreeMap<NaiveDate, DayStats> = BTreeMap::new();

    for interval in &db.intervals {
        let start_local = interval.start.with_timezone(&Local);
        let date = start_local.date_naive();
        let duration = interval.end - interval.start;

        let stats = daily_stats.entry(date).or_default();
        match interval.kind {
            IntervalType::Focus => {
                stats.total_focus = stats.total_focus + duration;
                stats.focus_sessions += 1;
            }
            IntervalType::Idle => {
                stats.total_idle = stats.total_idle + duration;
                stats.idle_sessions += 1;
            }
        }
    }

    Stats {
        daily_stats,
        today,
        week_start,
    }
}
