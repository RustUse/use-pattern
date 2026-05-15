# use-match

Reusable match-result primitives for RustUse pattern helpers.

> Warning: versions below `0.3.0` are experimental and may change as the workspace matures.

## Example Usage

```rust
use use_match::{MatchSpan, TextMatch, contains_match, match_len, slice_match};

let span = MatchSpan { start: 0, end: 4 };
let item = TextMatch {
    value: "rust".to_string(),
    span: span.clone(),
};

assert_eq!(match_len(&span), 4);
assert_eq!(slice_match("rustacean", &span), Some("rust"));
assert!(contains_match(&[item], "rust"));
```

## Scope

- string-backed span and match-result primitives
- safe span validation and slicing helpers
- small collection helpers for common match access patterns

## Non-Goals

- regex execution
- parser AST construction
- validation result frameworks

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
