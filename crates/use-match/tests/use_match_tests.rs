use use_match::{
    MatchSpan, TextMatch, contains_match, first_match, is_match_span_valid, last_match,
    match_is_empty, match_len, slice_match,
};

fn sample_matches() -> Vec<TextMatch> {
    vec![
        TextMatch {
            value: "alpha".to_string(),
            span: MatchSpan { start: 0, end: 5 },
        },
        TextMatch {
            value: "beta".to_string(),
            span: MatchSpan { start: 6, end: 10 },
        },
    ]
}

#[test]
fn validates_a_match_span() {
    let span = MatchSpan { start: 0, end: 4 };

    assert!(is_match_span_valid("rust", &span));
}

#[test]
fn rejects_invalid_match_spans() {
    assert!(!is_match_span_valid(
        "rust",
        &MatchSpan { start: 4, end: 2 }
    ));
    assert!(!is_match_span_valid(
        "rust",
        &MatchSpan { start: 0, end: 8 }
    ));
    assert!(!is_match_span_valid(
        "éclair",
        &MatchSpan { start: 1, end: 2 }
    ));
}

#[test]
fn slices_a_match() {
    let span = MatchSpan { start: 0, end: 4 };

    assert_eq!(slice_match("rustacean", &span), Some("rust"));
}

#[test]
fn handles_empty_match_spans() {
    let span = MatchSpan { start: 2, end: 2 };

    assert!(match_is_empty(&span));
    assert_eq!(slice_match("rust", &span), Some(""));
}

#[test]
fn reports_match_length() {
    assert_eq!(match_len(&MatchSpan { start: 2, end: 7 }), 5);
    assert_eq!(match_len(&MatchSpan { start: 7, end: 2 }), 0);
}

#[test]
fn returns_first_and_last_matches() {
    let matches = sample_matches();

    assert_eq!(
        first_match(&matches).map(|item| item.value.as_str()),
        Some("alpha")
    );
    assert_eq!(
        last_match(&matches).map(|item| item.value.as_str()),
        Some("beta")
    );
}

#[test]
fn detects_contained_match_values() {
    let matches = sample_matches();

    assert!(contains_match(&matches, "alpha"));
    assert!(!contains_match(&matches, "gamma"));
}

#[test]
fn handles_empty_input() {
    let matches: Vec<TextMatch> = Vec::new();
    let span = MatchSpan { start: 0, end: 0 };

    assert!(is_match_span_valid("", &span));
    assert_eq!(slice_match("", &span), Some(""));
    assert!(first_match(&matches).is_none());
    assert!(last_match(&matches).is_none());
    assert!(!contains_match(&matches, "anything"));
}
