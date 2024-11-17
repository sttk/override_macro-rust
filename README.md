# [override_macro-rust][repo-url] [![crate.io][crateio-img]][crateio-url] [![doc.rs][docrs-img]][docrs-url] [![CI Status][ci-img]][ci-url] [![MIT License][mit-img]][mit-url]

This crate provides attribute-like macros to override trait methods with other traits for a structs or another trait.

## Install

In `Cargo.toml`, write this crate as a dependency.

```toml
[dependencies]
override_macro = "0.1.0"
```

## Usage

First, by using `overridable` attribute macro, collects trait informations whose methods override methods of other trait and are overridden with methods of other traits.

The argument of this attribute macro is to specify the module path.
This argument is optional but it is better to specify it because the trait name may be conflict to other traits.

```rust
use override_macro::overridable;

#[overridable(mod = crate::module::path01)]
trait Trait1 {
    fn method1(&self, b: bool) -> u64;
}
```
```rust
use override_macro::overridable;

#[overridable(mod = crate::module::path02)]
trait Trait2 {
    fn method1(&self, b: bool) -> u64 {
        ...
    }
}
```

Next, by using `override_with` attribute macro, adds overridiing methods of the target trait for a struct or a trait.

The arguments of this attribute-macro are paths of traits having overriding methods.
This attribute-macro searches for methods with the same sigunature to the methods in the target trait from the traits passed as the arguments, and then adds these method callings to the target trait.

```rust
use override_macro::override_with;

struct StructA;
impl Trait02 for StructA {}

#[override_with(crate::module::path02::Trait2)]
impl Trait01 for StructA {}
```

## Supporting Rust versions

This crate supports Rust 1.80.1 or later.

```sh
% cargo msrv --no-check-feedback
Fetching index
Determining the Minimum Supported Rust Version (MSRV) for toolchain x86_64-apple-darwin
Using check command cargo check
   Finished The MSRV is: 1.80.1   ████████████████████████████████████████████ 00:00:36
```

## License

Copyright (C) 2024 Takayuki Sato

This program is free software under MIT License.<br>
See the file LICENSE in this distribution for more details.


[repo-url]: https://github.com/sttk/override_macro-rust
[crateio-img]: https://img.shields.io/badge/crate.io-ver.0.1.0-fc8d62?logo=rust
[crateio-url]: https://crates.io/crates/override_macro
[docrs-img]: https://img.shields.io/badge/doc.rs-override_macro-66c2a5?logo=docs.rs
[docrs-url]: https://docs.rs/override_macro
[ci-img]: https://github.com/sttk/override_macro-rust/actions/workflows/rust.yml/badge.svg?branch=main
[ci-url]: https://github.com/sttk/override_macro-rust/actions?query=branch%3Amain
[mit-img]: https://img.shields.io/badge/license-MIT-green.svg
[mit-url]: https://opensource.org/licenses/MIT
