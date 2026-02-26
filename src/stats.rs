use crate::models::{Database, IntervalType, Interval};
use chrono::{Datelike, Duration, Local, NaiveDate, DateTime, Utc};
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
    pub max_focus: Option<Duration>,
    pub min_focus: Option<Duration>,
    pub max_idle: Option<Duration>,
    pub min_idle: Option<Duration>,
}

pub struct Stats {
    pub daily_stats: BTreeMap<NaiveDate, DayStats>,
    pub session_summary: SummaryStats,
    pub today_summary: SummaryStats,
    pub week_summary: SummaryStats,
    pub today: NaiveDate,
    pub week_start: NaiveDate,
}

pub fn calculate_summary(intervals: &[Interval]) -> SummaryStats {
    let mut summary = SummaryStats::default();

    for interval in intervals {
        let duration = interval.end - interval.start;
        if duration < Duration::zero() {
            continue;
        }

        match interval.kind {
            IntervalType::Focus => {
                summary.total_focus += duration;
                summary.focus_count += 1;
                summary.max_focus = Some(summary.max_focus.map_or(duration, |m| m.max(duration)));
                summary.min_focus = Some(summary.min_focus.map_or(duration, |m| m.min(duration)));
            }
            IntervalType::Idle => {
                summary.total_idle += duration;
                summary.idle_count += 1;
                summary.max_idle = Some(summary.max_idle.map_or(duration, |m| m.max(duration)));
                summary.min_idle = Some(summary.min_idle.map_or(duration, |m| m.min(duration)));
            }
        }
    }

    summary
}

pub fn calculate_stats(db: &Database, run_start_time: Option<DateTime<Utc>>) -> Stats {
    let now_local = Local::now();
    let today = now_local.date_naive();

    // Find the start of the current week (Monday)
    let days_from_monday = now_local.weekday().num_days_from_monday();
    let week_start = today - Duration::days(days_from_monday as i64);
    let week_end = week_start + Duration::days(6);

    let mut daily_stats: BTreeMap<NaiveDate, DayStats> = BTreeMap::new();
    let mut session_intervals = Vec::new();
    let mut today_intervals = Vec::new();
    let mut week_intervals = Vec::new();

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

        if let Some(run_start) = run_start_time {
            if interval.start >= run_start {
                session_intervals.push(interval.clone());
            }
        }

        if date == today {
            today_intervals.push(interval.clone());
        }

        if date >= week_start && date <= week_end {
            week_intervals.push(interval.clone());
        }
    }

    Stats {
        daily_stats,
        session_summary: calculate_summary(&session_intervals),
        today_summary: calculate_summary(&today_intervals),
        week_summary: calculate_summary(&week_intervals),
        today,
        week_start,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Interval, IntervalType};
    use chrono::TimeZone;

    #[test]
    fn test_calculate_summary_empty() {
        let summary = calculate_summary(&[]);
        assert_eq!(summary.total_focus, Duration::zero());
        assert_eq!(summary.total_idle, Duration::zero());
        assert_eq!(summary.focus_count, 0);
        assert_eq!(summary.idle_count, 0);
        assert!(summary.max_focus.is_none());
    }

    #[test]
    fn test_calculate_summary_mixed() {
        let base_time = Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap();
        let intervals = vec![
            Interval {
                start: base_time,
                end: base_time + Duration::minutes(10),
                kind: IntervalType::Focus,
            },
            Interval {
                start: base_time + Duration::minutes(10),
                end: base_time + Duration::minutes(15),
                kind: IntervalType::Idle,
            },
            Interval {
                start: base_time + Duration::minutes(15),
                end: base_time + Duration::minutes(35),
                kind: IntervalType::Focus,
            },
        ];

        let summary = calculate_summary(&intervals);
        assert_eq!(summary.total_focus, Duration::minutes(30));
        assert_eq!(summary.total_idle, Duration::minutes(5));
        assert_eq!(summary.focus_count, 2);
        assert_eq!(summary.idle_count, 1);
        assert_eq!(summary.max_focus, Some(Duration::minutes(20)));
        assert_eq!(summary.min_focus, Some(Duration::minutes(10)));
    }

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
            ]
        };

        let stats = calculate_stats(&db, Some(run_start));

        // Session should only have the second interval
        assert_eq!(stats.session_summary.focus_count, 1);
        assert_eq!(stats.session_summary.total_focus, Duration::minutes(10));
    }
}
