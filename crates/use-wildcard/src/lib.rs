#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MatchToken {
    Literal(char),
    Star,
    Question,
}

fn tokenize_wildcard_pattern(pattern: &str) -> Vec<MatchToken> {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let mut tokens = Vec::new();
    let mut pattern_index = 0;

    while pattern_index < pattern_chars.len() {
        match pattern_chars[pattern_index] {
            '\\' => {
                if let Some(next_char) = pattern_chars.get(pattern_index + 1) {
                    tokens.push(MatchToken::Literal(*next_char));
                    pattern_index += 2;
                } else {
                    tokens.push(MatchToken::Literal('\\'));
                    pattern_index += 1;
                }
            },
            '*' => {
                tokens.push(MatchToken::Star);
                pattern_index += 1;
            },
            '?' => {
                tokens.push(MatchToken::Question);
                pattern_index += 1;
            },
            literal => {
                tokens.push(MatchToken::Literal(literal));
                pattern_index += 1;
            },
        }
    }

    tokens
}

fn escape_regex_char(character: char, output: &mut String) {
    match character {
        '.' | '+' | '*' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '^' | '$' | '\\' => {
            output.push('\\');
            output.push(character);
        },
        _ => output.push(character),
    }
}

fn wildcard_matches_impl(pattern: &str, input: &str) -> bool {
    let tokens = tokenize_wildcard_pattern(pattern);
    let input_chars: Vec<char> = input.chars().collect();
    let token_count = tokens.len();
    let input_count = input_chars.len();
    let mut matrix = vec![vec![false; input_count + 1]; token_count + 1];

    matrix[0][0] = true;

    for token_index in 1..=token_count {
        if matches!(tokens[token_index - 1], MatchToken::Star) {
            matrix[token_index][0] = matrix[token_index - 1][0];
        }
    }

    for token_index in 1..=token_count {
        for input_index in 1..=input_count {
            matrix[token_index][input_index] = match tokens[token_index - 1] {
                MatchToken::Literal(expected) => {
                    matrix[token_index - 1][input_index - 1]
                        && input_chars[input_index - 1] == expected
                },
                MatchToken::Question => matrix[token_index - 1][input_index - 1],
                MatchToken::Star => {
                    matrix[token_index - 1][input_index] || matrix[token_index][input_index - 1]
                },
            };
        }
    }

    matrix[token_count][input_count]
}

/// A simple wildcard pattern with optional case-sensitivity metadata.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WildcardPattern {
    pub pattern: String,
    pub case_sensitive: bool,
}

/// Returns `true` when the input contains an unescaped `*` or `?`.
pub fn has_wildcard(input: &str) -> bool {
    let tokens = tokenize_wildcard_pattern(input);

    tokens
        .iter()
        .any(|token| matches!(token, MatchToken::Star | MatchToken::Question))
}

/// Escapes wildcard metacharacters so they are treated literally.
pub fn escape_wildcard(input: &str) -> String {
    let mut escaped = String::new();

    for character in input.chars() {
        if matches!(character, '*' | '?' | '\\') {
            escaped.push('\\');
        }

        escaped.push(character);
    }

    escaped
}

/// Converts a wildcard pattern into an anchored regex string.
pub fn wildcard_to_regex(pattern: &str) -> String {
    let tokens = tokenize_wildcard_pattern(pattern);
    let mut regex_pattern = String::from("(?s)^");

    for token in tokens {
        match token {
            MatchToken::Literal(character) => escape_regex_char(character, &mut regex_pattern),
            MatchToken::Star => regex_pattern.push_str(".*"),
            MatchToken::Question => regex_pattern.push('.'),
        }
    }

    regex_pattern.push('$');
    regex_pattern
}

/// Returns `true` when the pattern matches the entire input.
pub fn wildcard_matches(pattern: &str, input: &str) -> bool {
    wildcard_matches_impl(pattern, input)
}

/// Returns `true` when the pattern matches the entire input after lowercasing both sides.
pub fn wildcard_matches_case_insensitive(pattern: &str, input: &str) -> bool {
    wildcard_matches_impl(&pattern.to_lowercase(), &input.to_lowercase())
}

/// Returns `true` when the first unescaped token is a wildcard.
pub fn starts_with_wildcard(pattern: &str) -> bool {
    matches!(
        tokenize_wildcard_pattern(pattern).first(),
        Some(MatchToken::Star | MatchToken::Question)
    )
}

/// Returns `true` when the last unescaped token is a wildcard.
pub fn ends_with_wildcard(pattern: &str) -> bool {
    matches!(
        tokenize_wildcard_pattern(pattern).last(),
        Some(MatchToken::Star | MatchToken::Question)
    )
}
