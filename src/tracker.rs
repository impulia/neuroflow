use crate::models::{Database, Interval, IntervalType};
use crate::storage::Storage;
use anyhow::Result;
use chrono::Utc;

pub struct Tracker {
    pub storage: Storage,
    pub threshold_secs: f64,
    pub db: Database,
    pub last_kind_seen: Option<IntervalType>,
    pub state_start: chrono::DateTime<Utc>,
    pub last_save: chrono::DateTime<Utc>,
}

impl Tracker {
    pub fn new(storage: Storage, threshold_mins: u64) -> Result<Self> {
        let db = storage.load()?;
        let now = Utc::now();
        Ok(Self {
            storage,
            threshold_secs: (threshold_mins * 60) as f64,
            db,
            last_kind_seen: None,
            state_start: now,
            last_save: now,
        })
    }

    pub fn tick(&mut self, idle_time: f64, now: chrono::DateTime<Utc>) -> Result<()> {
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
        Tracker::new(storage, 5).unwrap() // 5 mins threshold
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
}
