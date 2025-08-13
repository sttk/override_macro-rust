# [override_macro][repo-url] [![crates.io][cratesio-img]][cratesio-url] [![doc.rs][docrs-img]][docrs-url] [![CI Status][ci-img]][ci-url] [![MIT License][mit-img]][mit-url]

This crate provides attribute-like macros to override trait methods with other traits for a structs or another trait.

## Installation

In `Cargo.toml`, write this crate as a dependency.

```toml
[dependencies]
override_macro = "0.1.0"
```

## Usage

First, by using `overridable` attribute macro, collects trait informations whose methods override methods of other trait and are overridden with methods of other traits.

The argument of this attribute macro is to specify the module path.
This argument is optional but it is better to specify it because the trait name may be conflict to other traits.

Next, by using `override_with` attribute macro, adds overridiing methods of the target trait for a struct or a trait.

The arguments of this attribute-macro are paths of traits having overriding methods.
This attribute-macro searches for methods with the same sigunature to the methods in the target trait from the traits passed as the arguments, and then adds these method callings to the target trait.

```rust
use override_macro::{overridable, override_with};

#[overridable]
trait Trait0 {
    fn method0(&self) -> bool;
    fn method1(&self, b: bool) -> u64;
}

mod module_a {
    use override_macro::{overridable, override_with};

    #[overridable(mod = module_a)]
    pub trait Trait1 {
        fn method0(&self) -> bool { true }
    }

    pub mod module_b {
        use override_macro::{overridable, override_with};

        #[overridable(mod = module_a::module_b)]
        pub trait Trait2 {
            fn method1(&self, _b: bool) -> u64 { 123 }
        }
    }
}

struct Struct0;
impl module_a::Trait1 for Struct0 {}
impl module_a::module_b::Trait2 for Struct0 {}

#[override_with(module_a::Trait1, module_a::module_b::Trait2)]
impl Trait0 for Struct0 {
    // The following method is added automatically by this attribute-macro
    // fn method0(&self) -> bool {
    //     module_a::Trait1::method0(self)
    // }
    // fn method1(&self, _b: bool) -> u64 {
    //     module_a::module_b::Trait1::method1(self, _b)
    // }
}
```

## Supporting Rust versions

This crate supports Rust 1.80.1 or later.

```sh
% cargo msrv find
  [Meta]   cargo-msrv 0.18.4
       ~~~~~~(omission)~~~~~
Result:
   Considered (min … max):   Rust 1.56.1 … Rust 1.89.0
   Search method:            bisect
   MSRV:                     1.80.1
   Target:                   x86_64-apple-darwin
```

## License

Copyright (C) 2024-2025 Takayuki Sato

This program is free software under MIT License.<br>
See the file LICENSE in this distribution for more details.


[repo-url]: https://github.com/sttk/override_macro-rust
[cratesio-img]: https://img.shields.io/badge/crates.io-ver.0.1.0-fc8d62?logo=rust
[cratesio-url]: https://crates.io/crates/override_macro
[docrs-img]: https://img.shields.io/badge/doc.rs-override_macro-66c2a5?logo=docs.rs
[docrs-url]: https://docs.rs/override_macro
[ci-img]: https://github.com/sttk/override_macro-rust/actions/workflows/rust.yml/badge.svg?branch=main
[ci-url]: https://github.com/sttk/override_macro-rust/actions?query=branch%3Amain
[mit-img]: https://img.shields.io/badge/license-MIT-green.svg
[mit-url]: https://opensource.org/licenses/MIT
