use crate::models::{Database, IntervalType};
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, Utc};
use std::collections::BTreeMap;

#[derive(Default, Clone, Debug)]
pub struct DayStats {
    pub total_focus: Duration,
    pub total_idle: Duration,
    pub focus_sessions: u32,
    pub idle_sessions: u32,
}

#[derive(Default, Clone, Debug)]
pub struct SummaryStats {
    pub total_focus: Duration,
    pub total_idle: Duration,
    pub focus_count: u32,
    pub idle_count: u32,
}

pub struct Stats {
    pub daily_stats: BTreeMap<NaiveDate, DayStats>,
    pub session_summary: SummaryStats,
    pub today_summary: SummaryStats,
    pub week_summary: SummaryStats,
    pub today: NaiveDate,
    pub week_start: NaiveDate,
}

pub fn calculate_stats(db: &Database, run_start_time: Option<DateTime<Utc>>) -> Stats {
    let now_local = Local::now();
    let today = now_local.date_naive();

    // Find the start of the current week (Monday)
    let days_from_monday = now_local.weekday().num_days_from_monday();
    let week_start = today - Duration::days(days_from_monday as i64);
    let week_end = week_start + Duration::days(6);

    let mut daily_stats: BTreeMap<NaiveDate, DayStats> = BTreeMap::new();
    let mut session_summary = SummaryStats::default();
    let mut today_summary = SummaryStats::default();
    let mut week_summary = SummaryStats::default();

    for interval in &db.intervals {
        let start_local = interval.start.with_timezone(&Local);
        let date = start_local.date_naive();
        let duration = interval.end - interval.start;
        if duration < Duration::zero() {
            continue;
        }

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

        if let Some(run_start) = run_start_time {
            if interval.start >= run_start {
                update_summary(&mut session_summary, interval.kind, duration);
            }
        }

        if date == today {
            update_summary(&mut today_summary, interval.kind, duration);
        }

        if date >= week_start && date <= week_end {
            update_summary(&mut week_summary, interval.kind, duration);
        }
    }

    Stats {
        daily_stats,
        session_summary,
        today_summary,
        week_summary,
        today,
        week_start,
    }
}

fn update_summary(summary: &mut SummaryStats, kind: IntervalType, duration: Duration) {
    match kind {
        IntervalType::Focus => {
            summary.total_focus += duration;
            summary.focus_count += 1;
        }
        IntervalType::Idle => {
            summary.total_idle += duration;
            summary.idle_count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Interval, IntervalType};
    use chrono::TimeZone;

    #[test]
    fn test_calculate_stats_filtering() {
        let base_time = Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap();
        let run_start = base_time + Duration::minutes(15);

        let db = Database {
            intervals: vec![
                Interval {
                    start: base_time,
                    end: base_time + Duration::minutes(10),
                    kind: IntervalType::Focus,
                },
                Interval {
                    start: base_time + Duration::minutes(20),
                    end: base_time + Duration::minutes(30),
                    kind: IntervalType::Focus,
                },
            ],
        };

        let stats = calculate_stats(&db, Some(run_start));

        // Session should only have the second interval
        assert_eq!(stats.session_summary.focus_count, 1);
        assert_eq!(stats.session_summary.total_focus, Duration::minutes(10));
    }

    #[test]
    fn test_ongoing_interval() {
        let base_time = Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap();
        let mut db = Database {
            intervals: vec![Interval {
                start: base_time,
                end: base_time + Duration::seconds(1),
                kind: IntervalType::Focus,
            }],
        };

        // Simulating a tick updating the end time
        db.intervals[0].end = base_time + Duration::seconds(10);

        let stats = calculate_stats(&db, Some(base_time));
        assert_eq!(stats.session_summary.total_focus, Duration::seconds(10));
    }
}
