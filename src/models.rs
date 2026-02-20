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
    pub fn new_at(kind: IntervalType, at: DateTime<Utc>) -> Self {
        Self {
            start: at,
            end: at,
            kind,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Database {
    pub intervals: Vec<Interval>,
}
