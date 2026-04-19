# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.4.0 (2026 Apr 19)

### Breaking Changes

- **[anodized-core] `Condition` became `PreCondition`** - Preconditions are now represented as zero-argument closures.
- **[anodized-core] `Capture.alias` became `Capture.pat`** - Captures now bind with full Rust patterns, not only identifiers. A pattern must still be irrefutable.
- **[anodized-core] `Spec` internals changed** - `Spec` now tracks source span and exposes helper methods such as `is_empty` and `spec_err`.

### Added

- **[anodized] Trait specs support** - `#[spec]` on traits and trait `impl` blocks, with compile-fail checks for unsupported placements (#71).
- **[anodized-core] Patterns in `captures`** - Destructuring in captures is now supported, e.g. `captures: expr as (a, b)` (#100).
- **[anodized-fmt] UNSTABLE: New formatter package** - Added a dedicated formatter (CLI and library) for `#[spec]` attributes, including `--check` mode and TOML config support (#92).
- **[anodized-core] `SpecArgs` for permissive parsing** - Added a raw spec representation that can hold partially-valid specs for tooling workflows (#94).

### Fixed

- **[anodized] Preconditions no longer permit early function exit** - Preconditions are wrapped in closures to prevent `return` from escaping the instrumented function (#69).
- **[anodized] Captures no longer permit early function exit** - Each capture expression is evaluated in a closure to prevent `return` from escaping the instrumented function (#107).
- **[anodized-core] Improved `captures` diagnostics and parsing behavior** - Simplified parsing and clearer errors for invalid capture forms (#101).
- **[anodized-fmt] Preserve comments in formatted specs** - Comments inside `#[spec]` attributes are now preserved (with documented skip cases) (#106).

### Changed

- **[workspace] CI now checks formatting and lints** (#105).
- **[workspace] Minor maintenance and Clippy cleanups** (#99).
- **[workspace] VS Code rust-analyzer settings updates** - Added setup for selecting spec runtime behavior (#68, #95).

### Documentation

- **[anodized] Documented trait specs feature in README** (#104).
- **[anodized] Quickstart and Cargo feature docs updates** (#89, #91).
- **[workspace] Added crates.io typo redirect package/docs** (#67).

## 0.3.0 (2025 Dec 11)

### Breaking Changes

- **Fixed handling `return` statements in a `fn` body** - Returns no longer bypass runtime checks.
- **Must explicitly select a `runtime-*` behavior** - See below.

### Added

- Support for capturing entry-time values via `captures:`.
- Explicit `runtime-*` behavior settings: `check-and-panic`, `check-and-print`, `no-check`.

## 0.2.1 (2025 Aug 26)

### Breaking Changes

- **Renamed `#[contract]` to `#[spec]`** - The main attribute macro has been renamed from `contract` to `spec` to avoid confusion with blockchain smart contracts and improve discoverability (#23)
- **Renamed `Contract` type to `Spec`** - The exposed data type follows the same renaming for consistency

### Added

- Support for `#[cfg(...)]` attributes on individual conditions, allowing conditional compilation of runtime checks (#14)
- Support for array syntax in conditions - multiple conditions can now be specified as arrays: `requires: [cond1, cond2]` (#9)
- Improved error messages for misplaced macro attributes (#19)

### Changed

- Enforced parameter order in the `#[spec]` macro - conditions must now appear in the order: `requires`, `maintains`, `ensures` (#12)
- Improved internal parsing architecture (#15)
- Enhanced test coverage with unit tests for instrumentation (#18)

### Documentation

- Completely revised README with clearer examples and motivation (#21)
- Added project logo (#20)
- Clarified dual MIT/Apache-2.0 licensing (#13)

## 0.1.0 (2025 Aug 20)

Initial release
