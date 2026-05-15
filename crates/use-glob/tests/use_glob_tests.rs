use use_glob::{
    escape_glob, glob_matches, glob_to_regex, has_glob_wildcards, has_recursive_glob,
    is_glob_pattern, normalize_glob_separators, split_glob_segments,
};

#[test]
fn detects_glob_patterns() {
    assert!(is_glob_pattern("src/**/*.rs"));
    assert!(!is_glob_pattern("src/main.rs"));
}

#[test]
fn detects_glob_wildcards() {
    assert!(has_glob_wildcards("*.rs"));
    assert!(has_glob_wildcards("file-[a-z].txt"));
    assert!(!has_glob_wildcards("main.rs"));
}

#[test]
fn detects_recursive_globs() {
    assert!(has_recursive_glob("src/**/mod.rs"));
    assert!(!has_recursive_glob("src/*/mod.rs"));
}

#[test]
fn matches_single_star_patterns() {
    assert!(glob_matches("src/*.rs", "src/lib.rs"));
    assert!(!glob_matches("src/*.rs", "src/nested/lib.rs"));
}

#[test]
fn matches_double_star_patterns() {
    assert!(glob_matches("src/**/*.rs", "src/lib.rs"));
    assert!(glob_matches("src/**/*.rs", "src/nested/lib.rs"));
}

#[test]
fn matches_question_patterns() {
    assert!(glob_matches("file-?.txt", "file-a.txt"));
    assert!(!glob_matches("file-?.txt", "file-aa.txt"));
}

#[test]
fn matches_character_classes() {
    assert!(glob_matches("file-[ab].txt", "file-a.txt"));
    assert!(glob_matches("file-[a-z].txt", "file-g.txt"));
    assert!(!glob_matches("file-[ab].txt", "file-z.txt"));
}

#[test]
fn converts_globs_to_regex() {
    assert_eq!(glob_to_regex("src/**/*.rs"), r"(?s)^src/(?:.*/)?[^/]*\.rs$");
}

#[test]
fn normalizes_and_splits_segments() {
    assert_eq!(
        normalize_glob_separators(r"src\bin\main.rs"),
        "src/bin/main.rs"
    );
    assert_eq!(
        split_glob_segments(r"src\nested\main.rs"),
        vec![
            "src".to_string(),
            "nested".to_string(),
            "main.rs".to_string()
        ]
    );
}

#[test]
fn handles_malformed_input() {
    assert_eq!(escape_glob("file[1].txt"), r"file\[1\].txt");
    assert!(glob_matches(r"file\[1\].txt", "file[1].txt"));
    assert!(glob_matches("file[", "file["));
    assert_eq!(glob_to_regex("file["), r"(?s)^file\[$");
}

#[test]
fn handles_empty_input() {
    assert!(!is_glob_pattern(""));
    assert_eq!(split_glob_segments(""), Vec::<String>::new());
    assert!(glob_matches("", ""));
    assert!(glob_matches("*", ""));
    assert!(!glob_matches("?", ""));
}
