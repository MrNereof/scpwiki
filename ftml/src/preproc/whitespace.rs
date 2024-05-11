/*
 * preproc/whitespace.rs
 *
 * ftml - Library to parse Wikidot text
 * Copyright (C) 2019-2022 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

//! This performs the various miscellaneous substitutions that Wikidot does
//! in preparation for its parsing and handling processes. These are:
//! * Replacing DOS and legacy Mac newlines
//! * Trimming whitespace lines
//! * Concatenating lines that end with backslashes
//! * Convert tabs to four spaces
//! * Convert null characters to regular spaces
//! * Compress groups of 3+ newlines into 2 newlines

use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref WHITESPACE: Regex = {
        RegexBuilder::new(r"^\s+$")
            .multi_line(true)
            .build()
            .unwrap()
    };
    static ref LEADING_WHITESPACE: Regex = {
        RegexBuilder::new(r"^[ \u00a0]+")
            .multi_line(true)
            .build()
            .unwrap()
    };
    static ref TRAILING_WHITESPACE: Regex = {
        RegexBuilder::new(r"[ \u00a0]+$")
            .multi_line(true)
            .build()
            .unwrap()
    };
    static ref LEADING_NEWLINES: Regex = Regex::new(r"^\n+").unwrap();
    static ref TRAILING_NEWLINES: Regex = Regex::new(r"\n+$").unwrap();
}

pub fn substitute(text: &mut String) {
    // Replace DOS and Mac newlines
    str_replace(text, "\r\n", "\n");
    str_replace(text, "\r", "\n");

    // Strip lines with only whitespace
    regex_replace(text, &WHITESPACE, "");

    // Strip leading whitespace
    regex_replace(text, &LEADING_WHITESPACE, " ");

    // Strip trailing whitespace
    regex_replace(text, &TRAILING_WHITESPACE, "");

    // Join concatenated lines (ending with '\')
    str_replace(text, "\\\n", "");

    // Tabs to spaces
    str_replace(text, "\t", "    ");

    // Null characters to spaces
    str_replace(text, "\0", " ");

    // Remove leading and trailing newlines,
    // save one at the end
    regex_replace(text, &LEADING_NEWLINES, "");
    regex_replace(text, &TRAILING_NEWLINES, "");
}

fn str_replace(text: &mut String, pattern: &str, replacement: &str) {
    debug!(
        "Replacing miscellaneous static string (pattern {}, replacement {})",
        pattern, replacement,
    );

    while let Some(idx) = text.find(pattern) {
        let range = idx..idx + pattern.len();
        text.replace_range(range, replacement);
    }
}

fn regex_replace(text: &mut String, regex: &Regex, replacement: &str) {
    debug!(
        "Replacing miscellaneous regular expression (pattern {}, replacement {})",
        regex.as_str(),
        replacement,
    );

    let mut offset = 0;
    while let Some(mtch) = regex.find_at(text, offset) {
        let range = mtch.start()..mtch.end();
        let actual_count = text[range.start..range.end].chars().count();
        let mut actual_replacement = String::from("");
        for _ in 0..actual_count {
            actual_replacement.push_str(replacement);
        }
        offset = (mtch.end() as i32 + (replacement.len() as i32 - range.len() as i32)) as usize;
        text.replace_range(range, &actual_replacement);
    }
}