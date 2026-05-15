# Releasing

This workspace uses `release-plz` for changelog and release automation after the initial manual
publish wave.

## Initial Publish Order

1. `use-match`
2. `use-regex`
3. `use-glob`
4. `use-wildcard`
5. `use-pattern`

## Local Validation

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test --workspace --no-default-features
cargo check --workspace --all-features --examples
cargo doc --workspace --no-deps
```
