# use-pattern

Feature-gated facade crate for the RustUse pattern helper workspace.

> Warning: versions below `0.3.0` are experimental and may change as the workspace matures.

## Example Usage

```toml
[dependencies]
use-pattern = { version = "0.1", default-features = false, features = ["glob", "wildcard"] }
```

```rust
# #[cfg(all(feature = "glob", feature = "wildcard"))]
# fn main() {
use use_pattern::glob::is_glob_pattern;
use use_pattern::wildcard::wildcard_matches;

assert!(is_glob_pattern("src/**/*.rs"));
assert!(wildcard_matches("data-*.json", "data-01.json"));
# }
# #[cfg(not(all(feature = "glob", feature = "wildcard")))]
# fn main() {}
```

## Scope

- opt-in access to the focused `use-match`, `use-regex`, `use-glob`, and `use-wildcard` crates
- a small facade surface that does not force unused helpers into downstream builds
- straightforward reexports that mirror the concrete crate boundaries

## Non-Goals

- adding major new behavior beyond the focused crates
- hiding the underlying crate boundaries behind a framework-style abstraction
- replacing dedicated pattern, parsing, or filesystem ecosystems

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
