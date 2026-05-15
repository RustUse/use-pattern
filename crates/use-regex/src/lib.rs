#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use regex::Regex;

/// A reusable owned regex pattern wrapper.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RegexPattern {
    pub pattern: String,
}

/// Optional flags that can be applied when constructing a regex.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RegexFlags {
    pub case_insensitive: bool,
    pub multi_line: bool,
    pub dot_matches_new_line: bool,
}

/// A captured value and its byte span.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RegexCapture {
    pub name: Option<String>,
    pub value: String,
    pub start: usize,
    pub end: usize,
}

fn compile_regex(pattern: &str) -> Option<Regex> {
    Regex::new(pattern).ok()
}

fn compile_anchored(pattern: &str, anchor_start: bool, anchor_end: bool) -> Option<Regex> {
    let mut anchored = String::new();

    if anchor_start {
        anchored.push('^');
    }

    anchored.push_str("(?:");
    anchored.push_str(pattern);
    anchored.push(')');

    if anchor_end {
        anchored.push('$');
    }

    compile_regex(&anchored)
}

/// Escapes regex metacharacters in a literal string.
pub fn escape_regex(input: &str) -> String {
    regex::escape(input)
}

/// Returns `true` when the input appears to contain regex syntax.
pub fn looks_like_regex(input: &str) -> bool {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return false;
    }

    if trimmed.starts_with('/') && trimmed[1..].contains('/') {
        return true;
    }

    trimmed.contains([
        '\\', '^', '$', '.', '|', '?', '*', '+', '(', ')', '[', ']', '{', '}',
    ])
}

/// Returns `true` when the pattern compiles successfully.
pub fn is_valid_regex(input: &str) -> bool {
    compile_regex(input).is_some()
}

/// Returns `true` when the pattern matches any part of the input.
pub fn has_regex_match(pattern: &str, input: &str) -> bool {
    compile_regex(pattern)
        .map(|regex| regex.is_match(input))
        .unwrap_or(false)
}

/// Counts matches for a valid pattern.
pub fn count_regex_matches(pattern: &str, input: &str) -> Option<usize> {
    compile_regex(pattern).map(|regex| regex.find_iter(input).count())
}

/// Extracts matched substrings for a valid pattern.
pub fn extract_regex_matches(pattern: &str, input: &str) -> Option<Vec<String>> {
    compile_regex(pattern).map(|regex| {
        regex
            .find_iter(input)
            .map(|item| item.as_str().to_string())
            .collect()
    })
}

/// Extracts capture groups across all matches for a valid pattern.
pub fn extract_regex_captures(pattern: &str, input: &str) -> Option<Vec<RegexCapture>> {
    let regex = compile_regex(pattern)?;
    let capture_names: Vec<Option<String>> = regex
        .capture_names()
        .map(|name| name.map(ToOwned::to_owned))
        .collect();
    let mut values = Vec::new();

    for captures in regex.captures_iter(input) {
        for index in 1..captures.len() {
            if let Some(capture) = captures.get(index) {
                values.push(RegexCapture {
                    name: capture_names.get(index).cloned().flatten(),
                    value: capture.as_str().to_string(),
                    start: capture.start(),
                    end: capture.end(),
                });
            }
        }
    }

    Some(values)
}

/// Replaces all matches for a valid pattern.
pub fn replace_regex_all(pattern: &str, input: &str, replacement: &str) -> Option<String> {
    compile_regex(pattern).map(|regex| regex.replace_all(input, replacement).into_owned())
}

/// Returns `true` when a valid pattern matches at the beginning of the input.
pub fn regex_starts_with(pattern: &str, input: &str) -> bool {
    compile_anchored(pattern, true, false)
        .map(|regex| regex.is_match(input))
        .unwrap_or(false)
}

/// Returns `true` when a valid pattern matches at the end of the input.
pub fn regex_ends_with(pattern: &str, input: &str) -> bool {
    compile_anchored(pattern, false, true)
        .map(|regex| regex.is_match(input))
        .unwrap_or(false)
}
