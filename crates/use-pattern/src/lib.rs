#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "glob")]
pub use use_glob;
#[cfg(feature = "glob")]
pub use use_glob as glob;

#[cfg(feature = "match")]
pub use use_match;
#[cfg(feature = "match")]
pub use use_match as matchers;

#[cfg(feature = "regex")]
pub use use_regex;
#[cfg(feature = "regex")]
pub use use_regex as regex;

#[cfg(feature = "wildcard")]
pub use use_wildcard;
#[cfg(feature = "wildcard")]
pub use use_wildcard as wildcard;

pub mod prelude;
