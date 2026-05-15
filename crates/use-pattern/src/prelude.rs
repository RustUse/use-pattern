#[cfg(feature = "glob")]
pub use crate::glob::{GlobPattern, glob_matches, glob_to_regex, is_glob_pattern};

#[cfg(feature = "match")]
pub use crate::matchers::{MatchKind, MatchSpan, NamedMatch, TextMatch, slice_match};

#[cfg(feature = "regex")]
pub use crate::regex::{RegexCapture, RegexFlags, RegexPattern, has_regex_match};

#[cfg(feature = "wildcard")]
pub use crate::wildcard::{WildcardPattern, wildcard_matches};
