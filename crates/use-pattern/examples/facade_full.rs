use use_pattern::{glob, matchers, regex, wildcard};

fn main() {
    let span = matchers::MatchSpan { start: 0, end: 4 };

    assert_eq!(matchers::slice_match("rustacean", &span), Some("rust"));
    assert!(regex::has_regex_match(r"\d+", "v2"));
    assert!(glob::glob_matches("src/**/*.rs", "src/bin/main.rs"));
    assert!(wildcard::wildcard_matches("data-*.json", "data-1.json"));
}
