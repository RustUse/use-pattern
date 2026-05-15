#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A half-open span within a UTF-8 string.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MatchSpan {
    pub start: usize,
    pub end: usize,
}

/// A matched string value paired with its span.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TextMatch {
    pub value: String,
    pub span: MatchSpan,
}

/// A matched string value paired with an optional name and span.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NamedMatch {
    pub name: Option<String>,
    pub value: String,
    pub span: MatchSpan,
}

/// A conservative classification for how a match was found.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MatchKind {
    Exact,
    Prefix,
    Suffix,
    Contains,
    Pattern,
    #[default]
    Unknown,
}

/// Returns `true` when a span is ordered, in bounds, and aligned to character boundaries.
pub fn is_match_span_valid(input: &str, span: &MatchSpan) -> bool {
    span.start <= span.end
        && span.end <= input.len()
        && input.is_char_boundary(span.start)
        && input.is_char_boundary(span.end)
}

/// Returns a borrowed slice for a valid span.
pub fn slice_match<'a>(input: &'a str, span: &MatchSpan) -> Option<&'a str> {
    if is_match_span_valid(input, span) {
        input.get(span.start..span.end)
    } else {
        None
    }
}

/// Returns the span length using saturating subtraction.
pub fn match_len(span: &MatchSpan) -> usize {
    span.end.saturating_sub(span.start)
}

/// Returns `true` when the span length is zero.
pub fn match_is_empty(span: &MatchSpan) -> bool {
    match_len(span) == 0
}

/// Returns `true` when a match list contains the given value.
pub fn contains_match(matches: &[TextMatch], value: &str) -> bool {
    matches.iter().any(|item| item.value == value)
}

/// Returns the first match in a list.
pub fn first_match(matches: &[TextMatch]) -> Option<&TextMatch> {
    matches.first()
}

/// Returns the last match in a list.
pub fn last_match(matches: &[TextMatch]) -> Option<&TextMatch> {
    matches.last()
}
