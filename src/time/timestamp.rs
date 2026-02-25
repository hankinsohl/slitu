// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/24/2026.

use anyhow::{Error, Result};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Timestamp is a struct used to record and compare times.  Timestamp is based on UTC and is thus suitable for
/// comparing times obtained from different computers.  Timestamp uses serde to serialize/deserialize to/from
/// JSON.
#[derive(Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Timestamp {
    time: DateTime<Utc>,
}

impl AsRef<DateTime<Utc>> for Timestamp {
    fn as_ref(&self) -> &DateTime<Utc> { &self.time }
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

    /// Creates a Timestamp using JSON stored in path.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::open(path.as_ref())?;
        let mut reader = BufReader::new(file);
        Timestamp::from_reader(&mut reader)
    }

    /// Creates a Timestamp using JSON read from reader.
    pub fn from_reader(reader: &mut dyn Read) -> Result<Self, Error> {
        Ok(serde_json::from_reader(reader)?)
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