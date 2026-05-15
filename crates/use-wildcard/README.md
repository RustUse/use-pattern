# use-wildcard

Simple wildcard matching helpers for RustUse.

> Warning: versions below `0.3.0` are experimental and may change as the workspace matures.

## Example Usage

```rust
use use_wildcard::has_wildcard;

assert!(has_wildcard("report-*.txt"));
```

## Scope

- lightweight `*` and `?` matching helpers
- conservative escaping and regex-string conversion
- predictable string-based behavior for fixtures and tooling

## Non-Goals

- full glob syntax
- filesystem matching
- regex engine implementation
- shell expansion

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
