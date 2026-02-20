use crate::stats::calculate_stats;
use crate::storage::Storage;
use crate::utils::format_duration;
use anyhow::Result;
use chrono::Duration;

pub struct Reporter {
    storage: Storage,
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

        let stats_data = calculate_stats(&db);

        println!("Neflo Report");
        println!("============");

        let mut week_total_focus = Duration::zero();
        let mut week_total_idle = Duration::zero();
        let mut week_focus_sessions = 0;
        let mut week_idle_sessions = 0;

        for (date, stats) in &stats_data.daily_stats {
            if *date < stats_data.week_start {
                continue;
            }

            let is_today = *date == stats_data.today;
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

        println!("\nWeekly Summary (Starting Monday {})", stats_data.week_start);
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
