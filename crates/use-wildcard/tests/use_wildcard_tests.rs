use use_wildcard::{
    ends_with_wildcard, escape_wildcard, has_wildcard, starts_with_wildcard, wildcard_matches,
    wildcard_matches_case_insensitive, wildcard_to_regex,
};

#[test]
fn detects_wildcards() {
    assert!(has_wildcard("report-*.txt"));
    assert!(has_wildcard("item-?.json"));
    assert!(!has_wildcard("report.txt"));
}

#[test]
fn matches_star_patterns() {
    assert!(wildcard_matches("data-*.json", "data-01.json"));
    assert!(wildcard_matches("*", "anything"));
    assert!(!wildcard_matches("data-*.json", "logs-01.json"));
}

#[test]
fn matches_question_patterns() {
    assert!(wildcard_matches("file-?.txt", "file-a.txt"));
    assert!(!wildcard_matches("file-?.txt", "file-ab.txt"));
}

#[test]
fn respects_case_sensitive_matching() {
    assert!(wildcard_matches("Rust*", "RustUse"));
    assert!(!wildcard_matches("Rust*", "rustuse"));
}

#[test]
fn supports_case_insensitive_matching() {
    assert!(wildcard_matches_case_insensitive("Rust*", "rustuse"));
}

#[test]
fn escapes_wildcards() {
    assert_eq!(escape_wildcard("data-*.json?"), "data-\\*.json\\?");
    assert!(wildcard_matches("data-\\*.json", "data-*.json"));
}

#[test]
fn converts_patterns_to_regex() {
    assert_eq!(wildcard_to_regex("data-?.json"), r"(?s)^data-.\.json$");
    assert_eq!(wildcard_to_regex("data-*.json"), r"(?s)^data-.*\.json$");
}

#[test]
fn detects_start_and_end_wildcards() {
    assert!(starts_with_wildcard("*suffix"));
    assert!(ends_with_wildcard("prefix?"));
    assert!(!starts_with_wildcard(r"\*suffix"));
    assert!(!ends_with_wildcard(r"prefix\?"));
}

#[test]
fn handles_malformed_input() {
    assert!(!has_wildcard("file\\"));
    assert!(wildcard_matches("file\\", "file\\"));
    assert_eq!(wildcard_to_regex("file\\"), r"(?s)^file\\$");
}

#[test]
fn handles_empty_input() {
    assert!(!has_wildcard(""));
    assert!(wildcard_matches("", ""));
    assert!(wildcard_matches("*", ""));
    assert!(!wildcard_matches("?", ""));
}
