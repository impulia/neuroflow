use crate::models::IntervalType;
use crate::storage::Storage;
use anyhow::Result;
use chrono::{Datelike, Local, Duration, NaiveDate};
use std::collections::BTreeMap;

pub struct Reporter {
    storage: Storage,
}

#[derive(Default, Clone, Debug)]
pub struct DayStats {
    pub total_focus: Duration,
    pub total_idle: Duration,
    pub focus_sessions: u32,
    pub idle_sessions: u32,
}

pub struct ReportData {
    pub daily_stats: BTreeMap<NaiveDate, DayStats>,
    pub today: NaiveDate,
    pub week_start: NaiveDate,
}

impl Reporter {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }

    pub fn get_data(&self) -> Result<ReportData> {
        let db = self.storage.load()?;

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

        Ok(ReportData {
            daily_stats,
            today,
            week_start,
        })
    }

    pub fn report(&self) -> Result<()> {
        let data = self.get_data()?;
        if data.daily_stats.is_empty() {
            println!("No data recorded yet.");
            return Ok(());
        }

        let color_focus = "\x1b[32m";
        let color_idle = "\x1b[33m";
        let color_reset = "\x1b[0m";
        let color_bold = "\x1b[1m";

        println!("{}Neflo Report{}", color_bold, color_reset);
        println!("============");

        let mut week_total_focus = Duration::zero();
        let mut week_total_idle = Duration::zero();
        let mut week_focus_sessions = 0;
        let mut week_idle_sessions = 0;

        let max_duration = data.daily_stats.values()
            .map(|s| s.total_focus.max(s.total_idle))
            .max()
            .unwrap_or(Duration::hours(1));

        let bar_width = 40;

        for (date, stats) in &data.daily_stats {
            if *date < data.week_start {
                continue;
            }

            let is_today = *date == data.today;
            let date_str = if is_today {
                format!("{} (Today)", date)
            } else {
                date.to_string()
            };

            println!("\n{}Date: {}{}", color_bold, date_str, color_reset);

            let focus_bar = generate_bar(stats.total_focus, max_duration, bar_width);
            let idle_bar = generate_bar(stats.total_idle, max_duration, bar_width);

            println!("  Focus: {}[{}] {}{}", color_focus, focus_bar, format_duration(stats.total_focus), color_reset);
            println!("  Idle:  {}[{}] {}{}", color_idle, idle_bar, format_duration(stats.total_idle), color_reset);
            println!("  Interruptions: {}", stats.idle_sessions);

            if stats.focus_sessions > 0 {
                let avg_focus = stats.total_focus / (stats.focus_sessions as i32);
                println!("  Avg Focus Session: {}", format_duration(avg_focus));
            }
            if stats.idle_sessions > 0 {
                let avg_idle = stats.total_idle / (stats.idle_sessions as i32);
                println!("  Avg Interruption:  {}", format_duration(avg_idle));
            }

            week_total_focus = week_total_focus + stats.total_focus;
            week_total_idle = week_total_idle + stats.total_idle;
            week_focus_sessions += stats.focus_sessions;
            week_idle_sessions += stats.idle_sessions;
        }

        println!("\n{}Weekly Summary (Starting Monday {}){}", color_bold, data.week_start, color_reset);
        println!("-------------------------------------------");

        let week_max = week_total_focus.max(week_total_idle);
        let week_focus_bar = generate_bar(week_total_focus, week_max, bar_width);
        let week_idle_bar = generate_bar(week_total_idle, week_max, bar_width);

        println!("Total Focus: {}[{}] {}{}", color_focus, week_focus_bar, format_duration(week_total_focus), color_reset);
        println!("Total Idle:  {}[{}] {}{}", color_idle, week_idle_bar, format_duration(week_total_idle), color_reset);
        println!("Total Interruptions: {}", week_idle_sessions);

        if week_focus_sessions > 0 {
            let avg_focus = week_total_focus / (week_focus_sessions as i32);
            println!("Avg Focus Session:   {}", format_duration(avg_focus));
        }
        if week_idle_sessions > 0 {
            let avg_idle = week_total_idle / (week_idle_sessions as i32);
            println!("Avg Interruption:    {}", format_duration(avg_idle));
        }

        Ok(())
    }
}

fn generate_bar(duration: Duration, max_duration: Duration, width: usize) -> String {
    let ratio = if max_duration.num_seconds() == 0 {
        0.0
    } else {
        duration.num_seconds() as f64 / max_duration.num_seconds() as f64
    };
    let filled = (ratio * width as f64).round() as usize;
    let filled = filled.min(width);
    let mut bar = String::new();
    for i in 0..width {
        if i < filled {
            bar.push('█');
        } else {
            bar.push('░');
        }
    }
    bar
}

pub fn format_duration(d: Duration) -> String {
    let secs = d.num_seconds();
    let hours = secs / 3600;
    let mins = (secs % 3600) / 60;
    let secs = secs % 60;
    if hours > 0 {
        format!("{}h {}m {}s", hours, mins, secs)
    } else if mins > 0 {
        format!("{}m {}s", mins, secs)
    } else {
        format!("{}s", secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::seconds(30)), "30s");
        assert_eq!(format_duration(Duration::seconds(90)), "1m 30s");
        assert_eq!(format_duration(Duration::seconds(3661)), "1h 1m 1s");
    }
}
