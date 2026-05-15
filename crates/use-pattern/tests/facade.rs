#[test]
fn facade_builds_without_features() {
    assert_eq!(2 + 2, 4);
}

#[cfg(feature = "match")]
#[test]
fn match_feature_reexports_child_crate() {
    assert_eq!(
        use_pattern::matchers::match_len(&use_pattern::matchers::MatchSpan { start: 1, end: 3 }),
        2
    );
}

#[cfg(feature = "regex")]
#[test]
fn regex_feature_reexports_child_crate() {
    assert!(use_pattern::regex::has_regex_match(r"\d+", "value-42"));
}

#[cfg(feature = "glob")]
#[test]
fn glob_feature_reexports_child_crate() {
    assert!(use_pattern::glob::glob_matches("src/**/*.rs", "src/lib.rs"));
}

#[cfg(feature = "wildcard")]
#[test]
fn wildcard_feature_reexports_child_crate() {
    assert!(use_pattern::wildcard::wildcard_matches(
        "data-*.json",
        "data-1.json"
    ));
}

#[cfg(all(
    feature = "match",
    feature = "regex",
    feature = "glob",
    feature = "wildcard"
))]
#[test]
fn prelude_reexports_common_items() {
    use use_pattern::prelude::{MatchSpan, glob_matches, has_regex_match, wildcard_matches};

    assert_eq!(MatchSpan { start: 0, end: 2 }.end, 2);
    assert!(has_regex_match(r"\d+", "v2"));
    assert!(glob_matches("src/**/*.rs", "src/bin/main.rs"));
    assert!(wildcard_matches("data-*.json", "data-2.json"));
}
