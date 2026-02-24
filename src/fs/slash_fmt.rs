// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/24/2026.

use std::path::Path;

// path.join() uses backslash on Windows as the separator.  The SlashFmt trait enables us to reformat
// paths containing backslash into paths containing only slash to generate a uniform path display format.
pub trait SlashFmt {
    fn to_slash_fmt(&self) -> String;
}

impl SlashFmt for Path {
    fn to_slash_fmt(&self) -> String {
        self.to_string_lossy().replace("\\", "/")
    }
}
