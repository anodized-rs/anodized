[![crates.io](https://img.shields.io/crates/v/anodized.svg)](https://crates.io/crates/anodized)
[![docs.rs](https://docs.rs/anodized/badge.svg)](https://docs.rs/anodized)
[![CI tests](https://github.com/mkovaxx/anodized/actions/workflows/ci.yml/badge.svg)](https://github.com/mkovaxx/anodized/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/mkovaxx/anodized/blob/main/LICENSE-MIT)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/mkovaxx/anodized/blob/main/LICENSE-APACHE)

<img width="100" alt="Anodized Logo" src="https://raw.githubusercontent.com/mkovaxx/anodized/main/assets/logo.svg">

> Harden your Rust with **specifications**.

**The detailed reference is here: [The Anodized Reference](https://github.com/mkovaxx/anodized/blob/main/crates/anodized/REFERENCE.md).**

# Anodized

In short: `anodized` is to specification what `serde` is to serialization.

Anodized is a unified specification layer for Rust: it allows writing specs directly in Rust. The specs can **express complex properties** that go far beyond what the type system supports. Anodized **works on stable Rust** and does not alter the language or the toolchain in any way, staying compatible with components such as `rust-analyzer`. Besides expressing specs, Anodized also helps enforce them by **providing integration points** for tools such as fuzzers, property-based testing, formal verifiers, and so on.

## The `spec` Attribute

- **highly expressive**: Write pre/postconditions, loop invariants, and type refinements: all in the standard Rust you already know.
- **deeply integrated**: Syntax/type-checked by the compiler, and understood by `rust-analyzer` - no need for special components.
- **widely compatible**: Validate the specs with any combination of runtime checks, fuzzers, model checkers, or formal provers.

<img style="max-width:630px;" alt="editor integration demo" src="https://raw.githubusercontent.com/mkovaxx/anodized/main/assets/anodized-editor-integration.gif">

## Quickstart

**1. Add Anodized to your project.**

```toml
[dependencies]
anodized = { version = "0.4.0" }
```

**2. Extend your code with specs.**

Use the `#[spec]` attribute to attach preconditions and postconditions to functions, invariants to loops, and refinements to data types. Each _condition_ is a standard Rust expression that evaluates to `bool`.

```rust,no_run
use anodized::spec;

#[spec(
    requires: [
        part >= 0.0,
        part <= whole,
        whole > 0.0,
    ],
    ensures: [
        *output >= 0.0,
        *output <= 100.0,
    ],
)]
fn calculate_percentage(part: f64, whole: f64) -> f64 {
    100.0 * part / whole
}
```

**3. Validate your code against the specs.**

Use one or more enforcement tool.

The easiest is runtime checks, which Anodized provides out of the box.

All you need is tests that make function calls.

```rust
#[test]
fn percentage_25_over_100() {
    // This call satisfies the spec and runs fine.
    println!("25 out of 100 = {}%", calculate_percentage(25.0, 100.0));
}

#[test]
fn percentage_10_over_0() {
    // This call violates the precondition and will panic.
    println!("10 out of 0 = {}%", calculate_percentage(10.0, 0.0));
}
```

Use the `anodized_panic` setting to instrument the code with runtime checks.

```bash
RUSTFLAGS="--cfg anodized_panic" cargo test
```

A spec violation will cause a panic with a descriptive error message:

```text
thread 'main' panicked at 'Precondition failed: part <= whole', src/main.rs:17:5
```

For more details and other approaches, see [The Anodized Reference](https://github.com/mkovaxx/anodized/blob/main/crates/anodized/REFERENCE.md).

## Why Anodized

The Rust Team is building [native contract support](https://github.com/rust-lang/rust/issues/128044) into the language. We hope that learnings from Anodized will help their work, and we plan to offer a migration tool so that Anodized users can switch to Rust-native contracts as soon as they're ready.

Rust has many excellent verification tools: Aeneas, Creusot, Flux, Hax, Kani, Prusti, Verus, just to name a few. Their wider adoption is limited by the following key issues:

- Modifications to Rust (the language or the toolchain) make learning and use more difficult.
- Differences make using a combination of tools difficult and increase switching costs.
- Keeping modified components in sync with upstream Rust is more work for tool developers.

By adopting Anodized as a frontend, developers of verification systems can focus on the analysis itself and avoid duplicating the effort of defining and processing specs. Users can write their specs once, and gain access to a wide range of enforcement tools including runtime checks, fuzzing, verification, and more.

## Why Write Specs as Rust Code

A core design principle of Anodized is that a spec uses **standard Rust syntax**. This is a deliberate choice that provides key benefits over using a custom language.

- **The Language You Already Know**: No need to learn yet another language to write the specs. Write them in the one you already know: standard Rust. Call functions, use macros (like `matches!`), or write `if` and `match` expressions, and so on. As long as it syntax- and type-checks, it's good to go.

- **An Integral Part of Your Code**: Specs aren't special comments or strings; they are real Rust expressions, fully integrated with your code. The Rust compiler checks every spec for syntax and type errors, just like any other part of your code. If you misspell a variable, compare incompatible types, or make any other mistake, you'll get a familiar compiler error pointing directly to the spec element that needs fixing.

## Why "Spec" Instead of "Contract"

The choice of "specification" (or "spec") over "contract" is deliberate. While Design by Contract has a rich history, the term "contract" is now strongly associated with blockchain. This is particularly true in Rust, which has become a leading language for smart contract development.

This naming collision hurts discoverability. Searching for "Rust contract" yields many blockchain results, not just correctness tools.

Using "specification" instead:

- **Improves discoverability**: Developers find correctness tools when searching for them.
- **Reduces confusion**: The distinction from blockchain is immediately clear.
- **Maintains clarity**: "Specification" accurately describes these formal behavior annotations.

The term "spec" is already familiar from test specs, API specs, and formal specifications. It conveys the same meaning as Design by Contract while avoiding modern ambiguity.

## Prior Art and Motivation

The idea of adding contracts to Rust isn't new, and Anodized builds upon the great work and ideas from several other projects and discussions in the community. It is a fresh take with a strong focus on ergonomics and a forward-looking vision for an integrated ecosystem.

**The `contracts` Crate**

The most direct and popular predecessor is the [`contracts`](https://crates.io/crates/contracts) crate. It is a mature and feature-rich library that also provides `#[requires]`, `#[ensures]`, and `#[invariant]` attributes. It has been a major inspiration for Anodized.

Anodized differentiates itself with a few key design choices:

- **Unified Attribute**: Anodized uses a single, comprehensive `#[spec]` attribute, presenting each specification as one cohesive block.

- **Ergonomic Focus**: The design process has been heavily focused on refining the user-facing syntax (e.g. keyword choices, return value binding) to be as intuitive, approachable, and powerful as possible.

- **Ecosystem Vision**: While `contracts` is an excellent tool for runtime checking, Anodized is designed from the ground up to be a foundational layer for a wider ecosystem of diverse correctness tools.

**Other Crates**

Older crates like `libhoare` (a compiler plugin from before procedural macros were stabilized) and `dbc` explored similar ideas, proving the long-standing interest in Design by Contract within the Rust community. Anodized benefits from the modern procedural macro system, which allows for much better integration with the compiler and toolchain.

**Inspiration from Other Languages**

Anodized is also inspired by languages where contracts are a first-class feature, not just a library. Languages like [Whiley](https://whiley.org), [Eiffel](https://eiffel.org), and [Ada/SPARK](https://adacore.com/about-spark) demonstrate the power of deeply integrating formal specifications into the syntax, type system, and toolchain. The Anodized ecosystem begins with one library, but shares the great ambition of those languages: to bring a similar level of integration and ergonomic feel to Rust.

## License

Anodized is distributed under the terms of the MIT License and the Apache License (Version 2.0). Users can choose either license, and contributors must license their changes under both.

See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request.

## Technical Documentation

For detailed technical documentation including the formal specification grammar and runtime check implementation details, see the [`anodized-core`](https://docs.rs/anodized-core) documentation.
