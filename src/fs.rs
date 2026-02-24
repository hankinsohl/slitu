// Copyright (c) 2025 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 3/19/2025.

use std::fs::File;
use std::io::{BufRead, BufReader};
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

/// Compares two text files.  If the files are identical, Ok(()) is returned; otherwise a descriptive message
/// is returned as an Err result.
pub fn compare_text_files<P: AsRef<Path>>(
    p1: P,
    p2: P,
    filters: Option<&[&str]>,
) -> Result<(), String> {
    let path_1 = p1.as_ref();
    let path_2 = p2.as_ref();
    let f1 = File::open(path_1)
        .map_err(|err| format!("Error '{}' opening '{}'.", err, path_1.to_slash_fmt()))?;
    let f2 = File::open(path_2)
        .map_err(|err| format!("Error '{}' opening '{}'.", err, path_2.to_slash_fmt()))?;
    let r1 = BufReader::new(f1);
    let r2 = BufReader::new(f2);

    let mut line_number = 0;
    let mut lines_2 = r2.lines();
    'next_line: for l1 in r1.lines() {
        line_number += 1;
        let l1 = l1.map_err(|err| {
            format!(
                "Error '{}' reading line from '{}'.",
                err,
                path_1.to_slash_fmt()
            )
        })?;
        let l2 = lines_2
            .next()
            .ok_or_else(|| {
                format!(
                    "'{}' is shorter and contains {} lines.",
                    path_2.to_slash_fmt(),
                    line_number - 1
                )
            })?
            .map_err(|err| {
                format!(
                    "Error '{}' reading line from '{}'.",
                    err,
                    path_2.to_slash_fmt()
                )
            })?;

        if let Some(filters) = filters {
            for filter in filters {
                if l1.contains(filter) || l2.contains(filter) {
                    continue 'next_line;
                }
            }
        }

        if l1 != l2 {
            return Err(format!(
                "Mismatch at line {}:\n\t{}: '{}'\n\t{}: '{}'",
                line_number,
                path_1.to_slash_fmt(),
                l1,
                path_2.to_slash_fmt(),
                l2
            ));
        }
    }

    let l2 = lines_2.next();
    if l2.is_some() {
        return Err(format!(
            "'{}' is shorter and contains {} lines.",
            path_1.to_slash_fmt(),
            line_number
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn path_to(file_name: &str) -> PathBuf {
        Path::new("tests/assets").join(file_name)
    }

    #[test]
    fn compare_text_files_same_file_used_twice_generates_no_errors() {
        let result = compare_text_files(&path_to("file.json"), &path_to("file.json"), None);
        assert!(result.is_ok(), "{}", result.unwrap_err());
    }

    #[test]
    fn compare_text_files_identical_files_generate_no_errors() {
        let result = compare_text_files(
            &path_to("file.json"),
            &path_to("exact_copy_of_file.json"),
            None,
        );
        assert!(result.is_ok(), "{}", result.unwrap_err());
    }

    #[test]
    fn compare_text_files_file_1_longer_generates_error() {
        let result = compare_text_files(&path_to("longer.json"), &path_to("file.json"), None);
        assert!(
            !result.is_ok(),
            "Comparison with file 1 longer failed to generate error."
        );
        assert_eq!(
            result.unwrap_err(),
            "'tests/assets/file.json' is shorter and contains 11 lines."
        );
    }

    #[test]
    fn compare_text_files_file_2_longer_generates_error() {
        let result = compare_text_files(&path_to("file.json"), &path_to("longer.json"), None);
        assert!(
            !result.is_ok(),
            "Comparison with file 2 longer failed to generate error."
        );
        assert_eq!(
            result.unwrap_err(),
            "'tests/assets/file.json' is shorter and contains 11 lines."
        );
    }

    #[test]
    fn compare_text_files_file_1_shorter_generates_error() {
        let result = compare_text_files(&path_to("shorter.json"), &path_to("file.json"), None);
        assert!(
            !result.is_ok(),
            "Comparison with file 1 shorter failed to generate error."
        );
        assert_eq!(
            result.unwrap_err(),
            "'tests/assets/shorter.json' is shorter and contains 8 lines."
        );
    }

    #[test]
    fn compare_text_files_file_2_shorter_generates_error() {
        let result = compare_text_files(&path_to("file.json"), &path_to("shorter.json"), None);
        assert!(
            !result.is_ok(),
            "Comparison with file 2 shorter failed to generate error."
        );
        assert_eq!(
            result.unwrap_err(),
            "'tests/assets/shorter.json' is shorter and contains 8 lines."
        );
    }

    #[test]
    fn compare_text_files_with_file_1_missing_generates_error() {
        let result = compare_text_files(&path_to("missing.json"), &path_to("file.json"), None);
        assert!(
            !result.is_ok(),
            "Comparison with file 1 missing failed to generate error."
        );
        assert!(
            result
                .unwrap_err()
                .contains("opening 'tests/assets/missing.json'")
        );
    }

    #[test]
    fn compare_text_files_with_file_2_missing_generates_error() {
        let result = compare_text_files(&path_to("file.json"), &path_to("missing.json"), None);
        assert!(
            !result.is_ok(),
            "Comparison with file 2 missing failed to generate error."
        );
        assert!(
            result
                .unwrap_err()
                .contains("opening 'tests/assets/missing.json'")
        );
    }

    #[test]
    fn compare_text_files_with_file_1_binary_generates_error() {
        let result = compare_text_files(&path_to("binary_file.bin"), &path_to("file.json"), None);
        assert!(
            !result.is_ok(),
            "Comparison with file 1 binary failed to generate error."
        );
        assert!(
            result
                .unwrap_err()
                .contains("reading line from 'tests/assets/binary_file.bin'")
        );
    }

    #[test]
    fn compare_text_files_with_file_2_binary_generates_error() {
        let result = compare_text_files(&path_to("file.json"), &path_to("binary_file.bin"), None);
        assert!(
            !result.is_ok(),
            "Comparison with file 2 binary failed to generate error."
        );
        assert!(
            result
                .unwrap_err()
                .contains("reading line from 'tests/assets/binary_file.bin'")
        );
    }

    #[test]
    fn compare_text_files_with_file_1_dissimilar_generates_error() {
        let result = compare_text_files(&path_to("dissimilar.json"), &path_to("file.json"), None);
        assert!(
            !result.is_ok(),
            "Comparison with file 1 dissimilar failed to generate error."
        );
        assert!(result.unwrap_err().contains("Mismatch at line 3"));
    }

    #[test]
    fn compare_text_files_with_file_2_dissimilar_generates_error() {
        let result = compare_text_files(&path_to("file.json"), &path_to("dissimilar.json"), None);
        assert!(
            !result.is_ok(),
            "Comparison with file 1 dissimilar failed to generate error."
        );
        assert!(result.unwrap_err().contains("Mismatch at line 3"));
    }

    #[test]
    fn compare_dissimilar_ids_text_files_without_filter_to_skip_ids_generates_error() {
        let result = compare_text_files(
            &path_to("file_2.json"),
            &path_to("dissimilar_ids_2.json"),
            None,
        );
        assert!(
            !result.is_ok(),
            "Comparison of files with dissimilar ids without filter to skip ids failed to generate error."
        );
        assert!(result.unwrap_err().contains("Mismatch at line 3"));
    }

    #[test]
    fn compare_dissimilar_ids_text_files_using_filter_to_skip_ids_generates_no_error() {
        let result = compare_text_files(
            &path_to("file_2.json"),
            &path_to("dissimilar_ids_2.json"),
            Some(&["_id"]),
        );
        assert!(result.is_ok(), "{}", result.unwrap_err());
    }
}
