# use-regex

Practical regex helper utilities for RustUse.

> Warning: versions below `0.3.0` are experimental and may change as the workspace matures.

## Example Usage

```rust
use use_regex::{escape_regex, is_valid_regex};

assert_eq!(escape_regex("a+b"), "a\\+b");
assert!(is_valid_regex(r"\d+"));
```

## Scope

- lightweight wrappers around the mature `regex` crate
- simple validation, matching, extraction, and replacement helpers
- string-oriented utilities that stay small and predictable

## Non-Goals

- regex engine implementation
- parser generators
- validation frameworks
- security sandboxing

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
