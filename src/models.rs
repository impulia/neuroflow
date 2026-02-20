use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum IntervalType {
    Focus,
    Idle,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Interval {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub kind: IntervalType,
}

impl Interval {
    pub fn new(kind: IntervalType) -> Self {
        let now = Utc::now();
        Self {
            start: now,
            end: now,
            kind,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Database {
    pub intervals: Vec<Interval>,
}
