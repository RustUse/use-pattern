# RustUse use-pattern

Small, composable Rust 2024 helpers for reusable pattern matching primitives.

`use-pattern` groups lightweight crates for match-result primitives, regular expression helpers,
glob helpers, wildcard helpers, and pattern classification or escaping utilities. The workspace is
aimed at text tooling, docs tooling, fixtures, CLIs, route helpers, configuration helpers, and
application scaffolding where a full parser stack or routing engine would be excessive.

> Warning: crates in this workspace are experimental while they remain below `0.3.0`. Expect API
> refinement as the set matures.

## Crate List

- `use-pattern`: feature-gated umbrella crate for the set
- `use-match`: reusable span and match-result primitives
- `use-regex`: practical regex inspection, execution, extraction, and replacement helpers
- `use-glob`: predictable glob detection, conversion, and matching helpers
- `use-wildcard`: simple `*` and `?` matching helpers

## Scope

- small, composable pattern helpers with predictable string behavior
- lightweight wrappers around mature behavior where that is the right tradeoff
- reusable building blocks for docs tooling, validation tooling, CLI fixtures, static sites, and
  app scaffolding
- helper APIs that return `Option` or `bool` instead of panicking on malformed input

## Non-Goals

- replacing mature regex engines, parser generators, routing frameworks, or validation frameworks
- filesystem walking, shell expansion, or gitignore semantics
- validation policy logic or full validation result frameworks
- full parser ASTs or framework-style abstraction layers

## Relationship to Other RustUse Sets

- `use-text` covers words, tokens, lines, slugs, casing, and Markdown-oriented text utilities.
- `use-validate` covers validation rules, validation results, validation errors, and field or data
  validation helpers.
- `use-fs` should own filesystem paths, filenames, extensions, directories, and filesystem-specific
  glob usage rather than the reusable string helpers here.
- `use-web` should own route, URL, and broader web concerns when behavior becomes web-specific.

## Example Usage

```toml
[dependencies]
use-pattern = { version = "0.1", default-features = false, features = ["match", "regex"] }
```

```rust
# #[cfg(all(feature = "match", feature = "regex"))]
# fn main() {
use use_pattern::use_match::{MatchSpan, slice_match};
use use_pattern::use_regex::count_regex_matches;

let span = MatchSpan { start: 0, end: 4 };

assert_eq!(slice_match("rustacean", &span), Some("rust"));
assert_eq!(count_regex_matches(r"\d+", "v1 v20"), Some(2));
# }
# #[cfg(not(all(feature = "match", feature = "regex")))]
# fn main() {}
```

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
