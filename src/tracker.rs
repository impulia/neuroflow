use crate::models::{Database, Interval, IntervalType};
use crate::storage::Storage;
use anyhow::Result;
use chrono::{DateTime, Local, NaiveTime, Utc};

pub struct Tracker {
    pub storage: Storage,
    pub threshold_secs: f64,
    pub db: Database,
    pub last_kind_seen: Option<IntervalType>,
    pub state_start: DateTime<Utc>,
    pub last_save: DateTime<Utc>,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub duration: Option<chrono::Duration>,
    pub run_start_time: DateTime<Utc>,
    pub session_ended_saved: bool,
}

impl Tracker {
    pub fn new(
        storage: Storage,
        threshold_mins: u64,
        start_time: Option<String>,
        end_time: Option<String>,
        duration: Option<String>,
    ) -> Result<Self> {
        let db = storage.load()?;
        let now = Utc::now();

        let parsed_start_time = start_time
            .map(|s| NaiveTime::parse_from_str(&s, "%H:%M"))
            .transpose()?;
        let parsed_end_time = end_time
            .map(|s| NaiveTime::parse_from_str(&s, "%H:%M"))
            .transpose()?;
        let parsed_duration = duration
            .map(|s| -> Result<chrono::Duration> {
                let d = humantime::parse_duration(&s)?;
                Ok(chrono::Duration::from_std(d)?)
            })
            .transpose()?;

        let mut tracker = Self {
            storage,
            threshold_secs: (threshold_mins * 60) as f64,
            db,
            last_kind_seen: None,
            state_start: now,
            last_save: now,
            start_time: parsed_start_time,
            end_time: parsed_end_time,
            duration: parsed_duration,
            run_start_time: now,
            session_ended_saved: false,
        };
        tracker.prune_old_data();
        Ok(tracker)
    }

    pub fn should_track(&self, now: DateTime<Utc>) -> bool {
        if self.duration.is_some() {
            return true;
        }
        if let Some(st) = self.start_time {
            if now.with_timezone(&Local).time() < st {
                return false;
            }
        }
        true
    }

    pub fn should_stop(&self, now: DateTime<Utc>) -> bool {
        if let Some(duration) = self.duration {
            if now - self.run_start_time >= duration {
                return true;
            }
        } else if let Some(et) = self.end_time {
            if now.with_timezone(&Local).time() >= et {
                return true;
            }
        }
        false
    }

    pub fn tick(&mut self, idle_time: f64, now: DateTime<Utc>) -> Result<()> {
        let current_kind = if idle_time >= self.threshold_secs {
            IntervalType::Idle
        } else {
            IntervalType::Focus
        };

        // Update database
        self.update_db(current_kind, idle_time, now);

        // Handle state transition
        if Some(current_kind) != self.last_kind_seen {
            self.state_start = now;
            self.last_kind_seen = Some(current_kind);
            self.storage.save(&self.db)?;
            self.last_save = now;
        }

        // Save every 30 seconds
        if now - self.last_save > chrono::Duration::seconds(30) {
            self.prune_old_data();
            self.storage.save(&self.db)?;
            self.last_save = now;
        }

        Ok(())
    }

    pub fn reset(&mut self) -> Result<()> {
        self.db.intervals.clear();
        self.storage.save(&self.db)?;
        Ok(())
    }

    pub fn prune_old_data(&mut self) {
        let thirty_days_ago = Utc::now() - chrono::Duration::days(30);
        self.db.intervals.retain(|i| i.end > thirty_days_ago);
    }

    pub fn update_db(
        &mut self,
        current_kind: IntervalType,
        idle_time: f64,
        now: chrono::DateTime<Utc>,
    ) {
        let db = &mut self.db;
        let gap_threshold = chrono::Duration::seconds(10);

        if db.intervals.is_empty() {
            db.intervals.push(Interval::new_at(current_kind, now));
            return;
        }

        let last_idx = db.intervals.len() - 1;

        // If it's been a long time since the last update, start a new interval
        if now - db.intervals[last_idx].end > gap_threshold {
            db.intervals.push(Interval::new_at(current_kind, now));
            return;
        }

        if db.intervals[last_idx].kind == current_kind {
            db.intervals[last_idx].end = now;
        } else {
            // Transition
            if current_kind == IntervalType::Idle {
                // Focus -> Idle
                let idle_start = now - chrono::Duration::seconds(idle_time as i64);

                if idle_start <= db.intervals[last_idx].start {
                    // Backdated idle start is before or at the start of the current Focus interval.
                    // Convert the current interval to Idle.
                    db.intervals[last_idx].kind = IntervalType::Idle;
                    db.intervals[last_idx].end = now;
                } else {
                    // Split the interval
                    db.intervals[last_idx].end = idle_start;
                    let mut new_interval = Interval::new_at(IntervalType::Idle, now);
                    new_interval.start = idle_start;
                    new_interval.end = now;
                    db.intervals.push(new_interval);
                }
            } else {
                // Idle -> Focus
                db.intervals[last_idx].end = now;
                db.intervals
                    .push(Interval::new_at(IntervalType::Focus, now));
            }
        }

        // Cleanup: remove 0 or negative duration intervals if any (shouldn't really happen but for safety)
        db.intervals.retain(|i| i.end >= i.start);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::Storage;
    use chrono::TimeZone;
    use std::path::PathBuf;

    fn setup_tracker(path: PathBuf) -> Tracker {
        let storage = Storage::from_path(path);
        Tracker::new(storage, 5, None, None, None).unwrap() // 5 mins threshold
    }

    #[test]
    fn test_update_db_initial() {
        let mut tracker = setup_tracker(PathBuf::from("dummy"));
        tracker.db = Database::default();
        let now = Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap();

        tracker.update_db(IntervalType::Focus, 0.0, now);

        assert_eq!(tracker.db.intervals.len(), 1);
        assert_eq!(tracker.db.intervals[0].kind, IntervalType::Focus);
        assert_eq!(tracker.db.intervals[0].start, now);
        assert_eq!(tracker.db.intervals[0].end, now);
    }

    #[test]
    fn test_update_db_continuous() {
        let mut tracker = setup_tracker(PathBuf::from("dummy"));
        tracker.db = Database::default();
        let t1 = Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap();
        let t2 = t1 + chrono::Duration::seconds(5);

        tracker.update_db(IntervalType::Focus, 0.0, t1);
        tracker.update_db(IntervalType::Focus, 0.0, t2);

        assert_eq!(tracker.db.intervals.len(), 1);
        assert_eq!(tracker.db.intervals[0].start, t1);
        assert_eq!(tracker.db.intervals[0].end, t2);
    }

    #[test]
    fn test_update_db_transition_focus_to_idle_backdated() {
        let mut tracker = setup_tracker(PathBuf::from("dummy"));
        tracker.db = Database::default();
        let start = Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap();
        let mut now = start;

        // Focus for 300s, updating every 5s to stay under gap_threshold
        for _ in 0..60 {
            tracker.update_db(IntervalType::Focus, 0.0, now);
            now = now + chrono::Duration::seconds(5);
        }

        // Now at 10:05:00, we detect 300s idle.
        // idle_start = 10:05:00 - 300s = 10:00:00.
        tracker.update_db(IntervalType::Idle, 300.0, now);

        assert_eq!(tracker.db.intervals.len(), 1);
        assert_eq!(tracker.db.intervals[0].kind, IntervalType::Idle);
        assert_eq!(tracker.db.intervals[0].start, start);
        assert_eq!(tracker.db.intervals[0].end, now);
    }

    #[test]
    fn test_update_db_transition_focus_to_idle_split() {
        let mut tracker = setup_tracker(PathBuf::from("dummy"));
        tracker.db = Database::default();
        let start = Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap();
        let mut now = start;

        // Focus for 600s, updating every 5s
        for _ in 0..120 {
            tracker.update_db(IntervalType::Focus, 0.0, now);
            now = now + chrono::Duration::seconds(5);
        }

        // Now at 10:10:00, we detect 300s idle.
        // idle_start = 10:10:00 - 300s = 10:05:00.
        tracker.update_db(IntervalType::Idle, 300.0, now);

        assert_eq!(tracker.db.intervals.len(), 2);
        assert_eq!(tracker.db.intervals[0].kind, IntervalType::Focus);
        assert_eq!(
            tracker.db.intervals[0].end,
            start + chrono::Duration::seconds(300)
        );
        assert_eq!(tracker.db.intervals[1].kind, IntervalType::Idle);
        assert_eq!(
            tracker.db.intervals[1].start,
            start + chrono::Duration::seconds(300)
        );
        assert_eq!(tracker.db.intervals[1].end, now);
    }

    #[test]
    fn test_update_db_transition_idle_to_focus() {
        let mut tracker = setup_tracker(PathBuf::from("dummy"));
        tracker.db = Database::default();
        let t1 = Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap();
        let t2 = t1 + chrono::Duration::seconds(300);

        tracker.update_db(IntervalType::Idle, 300.0, t1);
        tracker.update_db(IntervalType::Focus, 0.0, t2);

        assert_eq!(tracker.db.intervals.len(), 2);
        assert_eq!(tracker.db.intervals[0].kind, IntervalType::Idle);
        assert_eq!(tracker.db.intervals[1].kind, IntervalType::Focus);
        assert_eq!(tracker.db.intervals[1].start, t2);
    }

    #[test]
    fn test_update_db_gap() {
        let mut tracker = setup_tracker(PathBuf::from("dummy"));
        tracker.db = Database::default();
        let t1 = Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap();
        let t2 = t1 + chrono::Duration::seconds(60); // 1 min gap (threshold is 10s)

        tracker.update_db(IntervalType::Focus, 0.0, t1);
        tracker.update_db(IntervalType::Focus, 0.0, t2);

        assert_eq!(tracker.db.intervals.len(), 2);
        assert_eq!(tracker.db.intervals[0].start, t1);
        assert_eq!(tracker.db.intervals[1].start, t2);
    }

    #[test]
    fn test_prune_old_data() {
        let mut tracker = setup_tracker(PathBuf::from("dummy"));
        tracker.db = Database::default();

        let old_date = Utc::now() - chrono::Duration::days(31);
        let recent_date = Utc::now() - chrono::Duration::days(29);

        tracker
            .db
            .intervals
            .push(Interval::new_at(IntervalType::Focus, old_date));
        tracker.db.intervals[0].end = old_date + chrono::Duration::seconds(60);

        tracker
            .db
            .intervals
            .push(Interval::new_at(IntervalType::Focus, recent_date));
        tracker.db.intervals[1].end = recent_date + chrono::Duration::seconds(60);

        assert_eq!(tracker.db.intervals.len(), 2);

        tracker.prune_old_data();

        assert_eq!(tracker.db.intervals.len(), 1);
        assert_eq!(tracker.db.intervals[0].start, recent_date);
    }

    #[test]
    fn test_should_track_start_time() {
        let storage = Storage::from_path(PathBuf::from("dummy"));
        let st = Some("09:00".to_string());
        let tracker = Tracker::new(storage, 5, st, None, None).unwrap();

        // 08:00 today
        let t1 = Utc::now().with_timezone(&Local);
        let t1 = t1
            .date_naive()
            .and_hms_opt(8, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
            .with_timezone(&Utc);
        assert!(!tracker.should_track(t1));

        // 10:00 today
        let t2 = Utc::now().with_timezone(&Local);
        let t2 = t2
            .date_naive()
            .and_hms_opt(10, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
            .with_timezone(&Utc);
        assert!(tracker.should_track(t2));
    }

    #[test]
    fn test_should_stop_end_time() {
        let storage = Storage::from_path(PathBuf::from("dummy"));
        let et = Some("17:00".to_string());
        let tracker = Tracker::new(storage, 5, None, et, None).unwrap();

        // 16:00 today
        let t1 = Utc::now().with_timezone(&Local);
        let t1 = t1
            .date_naive()
            .and_hms_opt(16, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
            .with_timezone(&Utc);
        assert!(!tracker.should_stop(t1));

        // 18:00 today
        let t2 = Utc::now().with_timezone(&Local);
        let t2 = t2
            .date_naive()
            .and_hms_opt(18, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
            .with_timezone(&Utc);
        assert!(tracker.should_stop(t2));
    }

    #[test]
    fn test_should_stop_duration() {
        let storage = Storage::from_path(PathBuf::from("dummy"));
        let duration = Some("1h".to_string());
        let mut tracker = Tracker::new(storage, 5, None, None, duration).unwrap();

        let start = Utc::now();
        tracker.run_start_time = start;

        assert!(!tracker.should_stop(start + chrono::Duration::minutes(30)));
        assert!(tracker.should_stop(start + chrono::Duration::minutes(90)));
    }

    #[test]
    fn test_duration_prevails_over_start_time() {
        let storage = Storage::from_path(PathBuf::from("dummy"));
        let st = Some("09:00".to_string());
        let duration = Some("1h".to_string());
        let tracker = Tracker::new(storage, 5, st, None, duration).unwrap();

        // 08:00 today - should track because timeout is set
        let t1 = Utc::now().with_timezone(&Local);
        let t1 = t1
            .date_naive()
            .and_hms_opt(8, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
            .with_timezone(&Utc);
        assert!(tracker.should_track(t1));
    }
}
