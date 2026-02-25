// Copyright (c) 2025 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 3/18/2025.

#[cfg(feature = "fs")]
pub use fs::{compare::compare_text_files, slash_fmt::SlashFmt};

#[cfg(feature = "time")]
pub use time::timestamp::Timestamp;

#[cfg(feature = "fs")]
pub mod fs;
#[cfg(feature = "time")]
pub mod time;
