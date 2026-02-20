use crate::models::{Database, Interval, IntervalType};
use crate::storage::Storage;
use crate::system::get_idle_time;
use anyhow::Result;
use chrono::Utc;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Tracker {
    storage: Storage,
    threshold_secs: f64,
}

impl Tracker {
    pub fn new(storage: Storage, threshold_mins: u64) -> Self {
        Self {
            storage,
            threshold_secs: (threshold_mins * 60) as f64,
        }
    }

    pub fn start(&self, running: Arc<AtomicBool>) -> Result<()> {
        let mut db = self.storage.load()?;

        println!("Neflo started. Press Ctrl+C to stop.");
        println!("Threshold: {} minutes", self.threshold_secs / 60.0);

        let mut last_save = Utc::now();
        let mut last_kind_seen = None;

        while running.load(Ordering::SeqCst) {
            let idle_time = get_idle_time();

            let current_kind = if idle_time >= self.threshold_secs {
                IntervalType::Idle
            } else {
                IntervalType::Focus
            };

            // Update database
            self.update_db(&mut db, current_kind, idle_time);

            // Save on transition or every 30 seconds
            let now = Utc::now();
            if Some(current_kind) != last_kind_seen
                || now - last_save > chrono::Duration::seconds(30)
            {
                self.storage.save(&db)?;
                last_save = now;
                last_kind_seen = Some(current_kind);
            }

            // Update UI
            let status = match current_kind {
                IntervalType::Focus => "\x1b[32mIn Flow\x1b[0m",
                IntervalType::Idle => "\x1b[33mIdle\x1b[0m",
            };
            print!("\rStatus: {} (Idle time: {:.0}s)    ", status, idle_time);
            io::stdout().flush()?;

            thread::sleep(Duration::from_secs(1));
        }

        println!("\nStopping tracker...");
        // Final update to make sure end time is set correctly
        let idle_time = get_idle_time();
        let current_kind = if idle_time >= self.threshold_secs {
            IntervalType::Idle
        } else {
            IntervalType::Focus
        };
        self.update_db(&mut db, current_kind, idle_time);
        self.storage.save(&db)?;
        Ok(())
    }

    fn update_db(&self, db: &mut Database, current_kind: IntervalType, idle_time: f64) {
        let now = Utc::now();
        let gap_threshold = chrono::Duration::seconds(10);

        if db.intervals.is_empty() {
            db.intervals.push(Interval::new(current_kind));
            return;
        }

        let last_idx = db.intervals.len() - 1;

        // If it's been a long time since the last update, start a new interval
        if now - db.intervals[last_idx].end > gap_threshold {
            db.intervals.push(Interval::new(current_kind));
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
                    let mut new_interval = Interval::new(IntervalType::Idle);
                    new_interval.start = idle_start;
                    new_interval.end = now;
                    db.intervals.push(new_interval);
                }
            } else {
                // Idle -> Focus
                db.intervals[last_idx].end = now;
                db.intervals.push(Interval::new(IntervalType::Focus));
            }
        }

        // Cleanup: remove 0 or negative duration intervals if any (shouldn't really happen but for safety)
        db.intervals.retain(|i| i.end >= i.start);
    }
}
