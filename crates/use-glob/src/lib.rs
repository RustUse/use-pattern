#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
enum ClassItem {
    Single(char),
    Range(char, char),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum MatcherToken {
    Literal(char),
    Star,
    DoubleStar,
    Question,
    CharacterClass(Vec<ClassItem>),
}

fn parse_character_class(
    pattern_chars: &[char],
    start_index: usize,
) -> Option<(Vec<ClassItem>, usize)> {
    let mut class_chars = Vec::new();
    let mut pattern_index = start_index + 1;

    while pattern_index < pattern_chars.len() {
        match pattern_chars[pattern_index] {
            ']' if !class_chars.is_empty() => {
                return Some((build_class_items(&class_chars), pattern_index + 1));
            }
            '\\' if pattern_index + 1 < pattern_chars.len() => {
                class_chars.push(pattern_chars[pattern_index + 1]);
                pattern_index += 2;
            }
            character => {
                class_chars.push(character);
                pattern_index += 1;
            }
        }
    }

    None
}

fn build_class_items(class_chars: &[char]) -> Vec<ClassItem> {
    let mut items = Vec::new();
    let mut class_index = 0;

    while class_index < class_chars.len() {
        if class_index + 2 < class_chars.len() && class_chars[class_index + 1] == '-' {
            items.push(ClassItem::Range(
                class_chars[class_index],
                class_chars[class_index + 2],
            ));
            class_index += 3;
        } else {
            items.push(ClassItem::Single(class_chars[class_index]));
            class_index += 1;
        }
    }

    items
}

fn parse_glob_pattern(pattern: &str) -> Vec<MatcherToken> {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let mut tokens = Vec::new();
    let mut pattern_index = 0;

    while pattern_index < pattern_chars.len() {
        match pattern_chars[pattern_index] {
            '\\' => {
                if let Some(next_char) = pattern_chars.get(pattern_index + 1) {
                    if matches!(next_char, '*' | '?' | '[' | ']' | '\\') {
                        tokens.push(MatcherToken::Literal(*next_char));
                        pattern_index += 2;
                    } else {
                        tokens.push(MatcherToken::Literal('/'));
                        pattern_index += 1;
                    }
                } else {
                    tokens.push(MatcherToken::Literal('/'));
                    pattern_index += 1;
                }
            }
            '*' => {
                if pattern_chars.get(pattern_index + 1) == Some(&'*') {
                    tokens.push(MatcherToken::DoubleStar);
                    pattern_index += 2;
                } else {
                    tokens.push(MatcherToken::Star);
                    pattern_index += 1;
                }
            }
            '?' => {
                tokens.push(MatcherToken::Question);
                pattern_index += 1;
            }
            '[' => {
                if let Some((class_items, next_index)) =
                    parse_character_class(&pattern_chars, pattern_index)
                {
                    tokens.push(MatcherToken::CharacterClass(class_items));
                    pattern_index = next_index;
                } else {
                    tokens.push(MatcherToken::Literal('['));
                    pattern_index += 1;
                }
            }
            literal => {
                tokens.push(MatcherToken::Literal(literal));
                pattern_index += 1;
            }
        }
    }

    tokens
}

fn escape_regex_char(character: char, output: &mut String) {
    match character {
        '.' | '+' | '*' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '^' | '$' | '\\' => {
            output.push('\\');
            output.push(character);
        }
        _ => output.push(character),
    }
}

fn render_class_character(character: char, output: &mut String) {
    match character {
        '\\' | ']' | '^' | '-' => {
            output.push('\\');
            output.push(character);
        }
        _ => output.push(character),
    }
}

fn render_character_class(items: &[ClassItem], output: &mut String) {
    output.push('[');

    for item in items {
        match item {
            ClassItem::Single(character) => render_class_character(*character, output),
            ClassItem::Range(start, end) => {
                render_class_character(*start, output);
                output.push('-');
                render_class_character(*end, output);
            }
        }
    }

    output.push(']');
}

fn class_matches(items: &[ClassItem], character: char) -> bool {
    if character == '/' {
        return false;
    }

    items.iter().any(|item| match item {
        ClassItem::Single(expected) => *expected == character,
        ClassItem::Range(start, end) => {
            let lower = (*start).min(*end);
            let upper = (*start).max(*end);
            (lower..=upper).contains(&character)
        }
    })
}

fn glob_matches_tokens(
    tokens: &[MatcherToken],
    input_chars: &[char],
    token_index: usize,
    input_index: usize,
    memo: &mut HashMap<(usize, usize), bool>,
) -> bool {
    if let Some(cached) = memo.get(&(token_index, input_index)) {
        return *cached;
    }

    let result = if token_index == tokens.len() {
        input_index == input_chars.len()
    } else {
        match &tokens[token_index] {
            MatcherToken::Literal(expected) => {
                input_chars.get(input_index) == Some(expected)
                    && glob_matches_tokens(
                        tokens,
                        input_chars,
                        token_index + 1,
                        input_index + 1,
                        memo,
                    )
            }
            MatcherToken::Question => {
                input_chars
                    .get(input_index)
                    .copied()
                    .filter(|character| *character != '/')
                    .is_some()
                    && glob_matches_tokens(
                        tokens,
                        input_chars,
                        token_index + 1,
                        input_index + 1,
                        memo,
                    )
            }
            MatcherToken::CharacterClass(items) => {
                input_chars
                    .get(input_index)
                    .copied()
                    .filter(|character| class_matches(items, *character))
                    .is_some()
                    && glob_matches_tokens(
                        tokens,
                        input_chars,
                        token_index + 1,
                        input_index + 1,
                        memo,
                    )
            }
            MatcherToken::Star => {
                glob_matches_tokens(tokens, input_chars, token_index + 1, input_index, memo)
                    || input_chars
                        .get(input_index)
                        .copied()
                        .filter(|character| *character != '/')
                        .is_some_and(|_| {
                            glob_matches_tokens(
                                tokens,
                                input_chars,
                                token_index,
                                input_index + 1,
                                memo,
                            )
                        })
            }
            MatcherToken::DoubleStar => {
                glob_matches_tokens(tokens, input_chars, token_index + 1, input_index, memo)
                    || matches!(
                        tokens.get(token_index + 1),
                        Some(MatcherToken::Literal('/'))
                    ) && glob_matches_tokens(
                        tokens,
                        input_chars,
                        token_index + 2,
                        input_index,
                        memo,
                    )
                    || input_chars.get(input_index).is_some_and(|_| {
                        glob_matches_tokens(tokens, input_chars, token_index, input_index + 1, memo)
                    })
            }
        }
    };

    memo.insert((token_index, input_index), result);
    result
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GlobPattern {
    pub pattern: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GlobToken {
    Literal(String),
    Star,
    DoubleStar,
    Question,
    CharacterClass(String),
}

/// Returns `true` when the input contains any unescaped glob syntax.
pub fn is_glob_pattern(input: &str) -> bool {
    has_glob_wildcards(input)
}

/// Escapes glob metacharacters so they are treated literally.
pub fn escape_glob(input: &str) -> String {
    let mut escaped = String::new();

    for character in input.chars() {
        if matches!(character, '*' | '?' | '[' | ']' | '\\') {
            escaped.push('\\');
        }

        escaped.push(character);
    }

    escaped
}

/// Returns `true` when the glob pattern matches the full input string.
pub fn glob_matches(pattern: &str, input: &str) -> bool {
    let normalized_input = normalize_glob_separators(input);
    let input_chars: Vec<char> = normalized_input.chars().collect();
    let tokens = parse_glob_pattern(pattern);
    let mut memo = HashMap::new();

    glob_matches_tokens(&tokens, &input_chars, 0, 0, &mut memo)
}

/// Converts a glob pattern into an anchored regex string.
pub fn glob_to_regex(pattern: &str) -> String {
    let tokens = parse_glob_pattern(pattern);
    let mut regex_pattern = String::from("(?s)^");
    let mut token_index = 0;

    while token_index < tokens.len() {
        if matches!(tokens.get(token_index), Some(MatcherToken::DoubleStar))
            && matches!(
                tokens.get(token_index + 1),
                Some(MatcherToken::Literal('/'))
            )
        {
            regex_pattern.push_str("(?:.*/)?");
            token_index += 2;
            continue;
        }

        match &tokens[token_index] {
            MatcherToken::Literal(character) => escape_regex_char(*character, &mut regex_pattern),
            MatcherToken::Star => regex_pattern.push_str("[^/]*"),
            MatcherToken::DoubleStar => regex_pattern.push_str(".*"),
            MatcherToken::Question => regex_pattern.push_str("[^/]"),
            MatcherToken::CharacterClass(items) => {
                render_character_class(items, &mut regex_pattern)
            }
        }

        token_index += 1;
    }

    regex_pattern.push('$');
    regex_pattern
}

/// Splits a normalized glob pattern on `/` boundaries.
pub fn split_glob_segments(pattern: &str) -> Vec<String> {
    let normalized = normalize_glob_separators(pattern);

    if normalized.is_empty() {
        Vec::new()
    } else {
        normalized.split('/').map(ToOwned::to_owned).collect()
    }
}

/// Returns `true` when a pattern contains `**` semantics.
pub fn has_recursive_glob(pattern: &str) -> bool {
    parse_glob_pattern(pattern)
        .iter()
        .any(|token| matches!(token, MatcherToken::DoubleStar))
}

/// Returns `true` when a pattern contains any wildcard or character-class token.
pub fn has_glob_wildcards(pattern: &str) -> bool {
    parse_glob_pattern(pattern)
        .iter()
        .any(|token| !matches!(token, MatcherToken::Literal(_)))
}

/// Normalizes Windows separators to forward slashes.
pub fn normalize_glob_separators(pattern: &str) -> String {
    pattern.replace('\\', "/")
}
