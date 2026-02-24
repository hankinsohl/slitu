// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/24/2026.

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

/// Timestamp is a struct used to record and compare times.  Timestamp is based on UTC and is thus suitable for
/// comparing times obtained from different computers.  Timestamp uses serde to serialize/deserialize to/from
/// JSON.
#[derive(Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Timestamp {
    pub time: DateTime<Utc>,
}

impl Default for Timestamp {
    /// Creates a Timestamp for the current time.
    fn default() -> Self {
        Timestamp::new()
    }
}

impl Timestamp {
    /// Creates a Timestamp for the current time.
    pub fn new() -> Self {
        Timestamp::create(Utc::now())
    }

    /// Creates a Timestamp for the given time.
    pub fn create(time: DateTime<Utc>) -> Self {
        Self {
            time,
        }
    }

    /// Returns true if this Timestamp is as new or newer than other.
    pub fn is_current(&self, other: &Timestamp) -> bool {
        self.time >= other.time
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_timestamp_comparison() {
        let ts1 = Timestamp::new();
        assert!(ts1 == ts1);
        assert!(ts1.is_current(&ts1));

        thread::sleep(Duration::from_millis(1));
        let ts2 = Timestamp::new();
        assert!(ts2 == ts2);
        assert!(ts2 > ts1);
        assert!(ts2.is_current(&ts2));
        assert!(ts2.is_current(&ts1));
        assert!(!ts1.is_current(&ts2));
    }

    #[test]
    fn test_timestamp_serialization() {
        let ts1 = Timestamp::new();
        let ts1_json = serde_json::to_string(&ts1).unwrap();
        let ts1_deserialized: Timestamp = serde_json::from_str(&ts1_json).unwrap();

        thread::sleep(Duration::from_millis(1));
        let ts2 = Timestamp::new();
        let ts2_json = serde_json::to_string(&ts2).unwrap();
        let ts2_deserialized: Timestamp = serde_json::from_str(&ts2_json).unwrap();

        assert!(ts1 == ts1_deserialized);
        assert!(ts2 == ts2_deserialized);

        assert!(ts1_deserialized.is_current(&ts1_deserialized));
        assert!(ts2_deserialized == ts2_deserialized);
        assert!(ts2_deserialized > ts1_deserialized);
        assert!(ts2_deserialized.is_current(&ts2_deserialized));
        assert!(ts2_deserialized.is_current(&ts1_deserialized));
        assert!(!ts1_deserialized.is_current(&ts2_deserialized));
    }
}