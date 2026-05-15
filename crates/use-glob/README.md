# use-glob

Practical glob pattern helpers for RustUse.

> Warning: versions below `0.3.0` are experimental and may change as the workspace matures.

## Example Usage

```rust
use use_glob::is_glob_pattern;

assert!(is_glob_pattern("src/**/*.rs"));
```

## Scope

- string-based glob detection, conversion, and matching helpers
- normalized separator handling for predictable behavior
- lightweight utilities for tooling and fixtures

## Non-Goals

- filesystem walking
- gitignore engines
- shell expansion
- full platform-specific path semantics

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
