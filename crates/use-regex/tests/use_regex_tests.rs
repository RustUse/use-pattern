use use_regex::{
    RegexCapture, count_regex_matches, escape_regex, extract_regex_captures, extract_regex_matches,
    has_regex_match, is_valid_regex, looks_like_regex, regex_ends_with, regex_starts_with,
    replace_regex_all,
};

#[test]
fn escapes_regex_metacharacters() {
    assert_eq!(escape_regex("a+b?(test)"), "a\\+b\\?\\(test\\)");
}

#[test]
fn detects_valid_regex_patterns() {
    assert!(looks_like_regex(r"^item-\d+$"));
    assert!(is_valid_regex(r"^item-\d+$"));
}

#[test]
fn detects_invalid_regex_patterns() {
    assert!(!is_valid_regex("[abc"));
}

#[test]
fn detects_matches() {
    assert!(has_regex_match(r"\d+", "version 42"));
    assert!(!has_regex_match(r"\d+", "version forty-two"));
}

#[test]
fn counts_matches() {
    assert_eq!(count_regex_matches(r"\d+", "a1 b22 c333"), Some(3));
}

#[test]
fn extracts_matches() {
    assert_eq!(
        extract_regex_matches(r"\d+", "a1 b22 c333"),
        Some(vec!["1".to_string(), "22".to_string(), "333".to_string()])
    );
}

#[test]
fn extracts_captures() {
    assert_eq!(
        extract_regex_captures(r"(?P<kind>[a-z]+)-(\d+)", "item-42 part-7"),
        Some(vec![
            RegexCapture {
                name: Some("kind".to_string()),
                value: "item".to_string(),
                start: 0,
                end: 4,
            },
            RegexCapture {
                name: None,
                value: "42".to_string(),
                start: 5,
                end: 7,
            },
            RegexCapture {
                name: Some("kind".to_string()),
                value: "part".to_string(),
                start: 8,
                end: 12,
            },
            RegexCapture {
                name: None,
                value: "7".to_string(),
                start: 13,
                end: 14,
            },
        ])
    );
}

#[test]
fn replaces_matches() {
    assert_eq!(
        replace_regex_all(r"\d+", "v1 v20 v300", "#"),
        Some("v# v# v#".to_string())
    );
}

#[test]
fn checks_start_and_end_anchors() {
    assert!(regex_starts_with(r"item-\d+", "item-42 suffix"));
    assert!(!regex_starts_with(r"item-\d+", "prefix item-42"));
    assert!(regex_ends_with(r"item-\d+", "prefix item-42"));
    assert!(!regex_ends_with(r"item-\d+", "item-42 suffix"));
}

#[test]
fn handles_malformed_input() {
    assert!(!has_regex_match("(", "value"));
    assert_eq!(count_regex_matches("(", "value"), None);
    assert_eq!(extract_regex_matches("(", "value"), None);
    assert_eq!(extract_regex_captures("(", "value"), None);
    assert_eq!(replace_regex_all("(", "value", "x"), None);
    assert!(!regex_starts_with("(", "value"));
    assert!(!regex_ends_with("(", "value"));
}

#[test]
fn handles_empty_input() {
    assert!(!looks_like_regex(""));
    assert!(is_valid_regex(""));
    assert!(!has_regex_match(r"\d+", ""));
    assert_eq!(count_regex_matches(r"\d+", ""), Some(0));
    assert_eq!(extract_regex_matches(r"\d+", ""), Some(Vec::new()));
    assert_eq!(extract_regex_captures(r"(\d+)", ""), Some(Vec::new()));
    assert_eq!(replace_regex_all(r"\d+", "", "x"), Some(String::new()));
}
