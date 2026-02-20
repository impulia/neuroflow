use crate::models::IntervalType;
use crate::storage::Storage;
use crate::utils::format_duration;
use anyhow::Result;
use chrono::{Datelike, Duration, Local};
use std::collections::BTreeMap;

pub struct Reporter {
    storage: Storage,
}

#[derive(Default)]
struct DayStats {
    total_focus: Duration,
    total_idle: Duration,
    focus_sessions: u32,
    idle_sessions: u32,
}

impl Reporter {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }

    pub fn report(&self) -> Result<()> {
        let db = self.storage.load()?;
        if db.intervals.is_empty() {
            println!("No data recorded yet.");
            return Ok(());
        }

        let now_local = Local::now();
        let today_local = now_local.date_naive();

        // Find the start of the current week (Monday)
        let days_from_monday = now_local.weekday().num_days_from_monday();
        let week_start = today_local - Duration::days(days_from_monday as i64);

        let mut daily_stats: BTreeMap<chrono::NaiveDate, DayStats> = BTreeMap::new();

        for interval in &db.intervals {
            let start_local = interval.start.with_timezone(&Local);

            let date = start_local.date_naive();
            let duration = interval.end - interval.start;

            let stats = daily_stats.entry(date).or_default();
            match interval.kind {
                IntervalType::Focus => {
                    stats.total_focus += duration;
                    stats.focus_sessions += 1;
                }
                IntervalType::Idle => {
                    stats.total_idle += duration;
                    stats.idle_sessions += 1;
                }
            }
        }

        println!("Neflo Report");
        println!("============");

        let mut week_total_focus = Duration::zero();
        let mut week_total_idle = Duration::zero();
        let mut week_focus_sessions = 0;
        let mut week_idle_sessions = 0;

        for (date, stats) in &daily_stats {
            if *date < week_start {
                continue;
            }

            let is_today = *date == today_local;
            let date_str = if is_today {
                format!("{} (Today)", date)
            } else {
                date.to_string()
            };

            println!("\nDate: {}", date_str);
            println!(
                "  Focus Time:        {}",
                format_duration(stats.total_focus.num_seconds())
            );
            println!(
                "  Idle Time:         {}",
                format_duration(stats.total_idle.num_seconds())
            );
            println!("  Interruptions:     {}", stats.idle_sessions);

            if stats.focus_sessions > 0 {
                let avg_focus = stats.total_focus / (stats.focus_sessions as i32);
                println!(
                    "  Avg Focus Session: {}",
                    format_duration(avg_focus.num_seconds())
                );
            }
            if stats.idle_sessions > 0 {
                let avg_idle = stats.total_idle / (stats.idle_sessions as i32);
                println!(
                    "  Avg Interruption:  {}",
                    format_duration(avg_idle.num_seconds())
                );
            }

            week_total_focus += stats.total_focus;
            week_total_idle += stats.total_idle;
            week_focus_sessions += stats.focus_sessions;
            week_idle_sessions += stats.idle_sessions;
        }

        println!("\nWeekly Summary (Starting Monday {})", week_start);
        println!("-------------------------------------------");
        println!(
            "Total Focus Time:    {}",
            format_duration(week_total_focus.num_seconds())
        );
        println!(
            "Total Idle Time:     {}",
            format_duration(week_total_idle.num_seconds())
        );
        println!("Total Interruptions: {}", week_idle_sessions);
        if week_focus_sessions > 0 {
            let avg_focus = week_total_focus / (week_focus_sessions as i32);
            println!(
                "Avg Focus Session:   {}",
                format_duration(avg_focus.num_seconds())
            );
        }
        if week_idle_sessions > 0 {
            let avg_idle = week_total_idle / (week_idle_sessions as i32);
            println!(
                "Avg Interruption:    {}",
                format_duration(avg_idle.num_seconds())
            );
        }

        Ok(())
    }
}
