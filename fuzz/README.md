# Fuzz Targets for Testing Anodized Components

## fmt_ws_invariant

Tests that spec formatting is invariant to whitespace changes.

**IMPORTANT**: Due to `proc-macro2` special-casing its behavior based on `cfg(fuzzing)` which `cargo-fuzz` sets by default, it must be run with the `--no-cfg-fuzzing` option as follows:

```sh
cargo +nightly fuzz run --no-cfg-fuzzing fmt_ws_invariant
```
