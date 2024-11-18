// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

//! This crate provides attribute-like macros to override trait methods with other traits for
//! a structs or another trait.
//!
//! ## Install
//!
//! In `Cargo.toml`, write this crate as a dependency.
//!
//! ```toml
//! [dependencies]
//! override_macro = "0.1.0"
//! ```
//!
//! ## Usage
//!
//! First, by using `overridable` attribute-macro, collects trait information whose methods
//! override other trait's methods and are overridden with methods of other traits.
//!
//! The argument of this attribute-macro is to specify the module path.
//! This argument is optional but it is better to specify it because the trait name may be conflict
//! to other traits.
//!
//! Next, by using `override_with` attribute-macro, adds overridiing methods of the target trait
//! for a struct or a trait.
//!
//! The arguments of this attribute-macro are paths of traits having overriding methods.
//! This attribute-macro searches for methods with the same sigunature to the methods in the target
//! trait from the traits passed as the arguments, and then adds these method callings to the target
//! trait.
//!
//! ```rust
//! use override_macro::{overridable, override_with};
//!
//! #[overridable]
//! trait Trait0 {
//!     fn method0(&self) -> bool;
//!     fn method1(&self, b: bool) -> u64;
//! }
//!
//! mod module_a {
//!     use override_macro::{overridable, override_with};
//!
//!     #[overridable(mod = module_a)]
//!     pub trait Trait1 {
//!         fn method0(&self) -> bool { true }
//!     }
//!
//!     pub mod module_b {
//!         use override_macro::{overridable, override_with};
//!
//!         #[overridable(mod = module_a::module_b)]
//!         pub trait Trait2 {
//!             fn method1(&self, _b: bool) -> u64 { 123 }
//!         }
//!     }  
//! }  
//!
//! struct Struct0;
//! impl module_a::Trait1 for Struct0 {}
//! impl module_a::module_b::Trait2 for Struct0 {}
//!
//! #[override_with(module_a::Trait1, module_a::module_b::Trait2)]
//! impl Trait0 for Struct0 {
//!     // The following method is added automatically by this attribute-macro
//!     // fn method0(&self) -> bool {
//!     //     module_a::Trait1::method0(self)
//!     // }
//!     // fn method1(&self, _b: bool) -> u64 {
//!     //     module_a::module_b::Trait1::method1(self, _b)
//!     // }
//! }
//! ```

use proc_macro::TokenStream;

mod logic;
mod syn_dax;

/// Collects trait informations whose methods can override other trait methods, or can be overriden
/// with other trait methods.
///
/// This attribute-macro is attached to a trait definition block, and collects its name and its
/// method signatures.
/// The collected informations are used in `override_with` attribute-macro.
///
/// Internally, the trait information is managed using the trait name as the key, but there might
/// be other traits with the same name in different modules.
/// To treat such name conflicts, this macro can accept a module path as its arguments in the form
/// of `(mod module::path)`.
///
/// # Example
///
/// ```rust
/// use override_macro::overridable;
///
/// #[overridable]
/// trait TraitA {
///    /* ... */
/// }
///
/// #[overridable(mod = mod1::mod2)]
/// trait TraitB {
///    /* ... */
/// }
/// ```
#[proc_macro_attribute]
pub fn overridable(args: TokenStream, item: TokenStream) -> TokenStream {
    let dax = syn_dax::OverridableDax::new(args, item.clone());
    logic::collect_trait_info(&dax);

    item
}

/// Adds methods *overridden* methods with methods of other traits to the trait implementation.
///
/// This attribute-macro is attached to the `impl` block of the target trait for a struct or
/// another trait.
/// This searches for default implementations of methods with the same signature as the target
/// trait's methods from the traits specified in this macro's arguments.
/// And then, this adds method implementations to the `impl` block that call those default
/// implementations.
///
/// The target trait and the traits specified in the arguments of this attribute-macro must have
/// the `overridable` attribute-macro attached to their definition blocks.
///
/// If a method in the target trait has a default implemention, the overriding for the method
/// is skipped.
/// Therefore, if you want to implement a different process without overriding, or if multiple
/// traits in the attribute-macro arguments have the same method's default implementation,
/// you can directly implement it in the `impl` block.
///
/// # Example
///
/// ```rust
/// use override_macro::{overridable, override_with};
///
/// #[overridable]
/// trait Trait0 {
///     fn method0(&self, b: bool) { /* ... */ }
/// }
///
/// #[overridable]
/// trait Trait1 {
///     fn method1_a(&self, i: i32) { /* ... */ }
///     fn method1_b(&self, s: &str) { /* ... */ }
/// }
///
/// #[overridable]
/// trait Trait2 {
///     fn method0(&self, b: bool);
///     fn method1_a(&self, i: i32);
///     fn method1_b(&self, s: &str);
/// }
///
/// struct StructA;
/// impl Trait0 for StructA {}
/// impl Trait1 for StructA {}
///
/// #[override_with(Trait0, Trait1)]
/// impl Trait2 for StructA {
///     /* The following methods are automatically added by this attribute-macro.
///     fn method0(&self, b: bool) { Trait0::method0(self, b) }
///     fn method1_a(&self, i: i32) { Trait1::method1_a(self, i) }
///     */
///     // Skip to override method of which default implementation exists.
///     fn method1_b(&self, s: &str) { /* ... */ }
/// }
/// ```
#[proc_macro_attribute]
pub fn override_with(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut dax = syn_dax::OverrideWithDax::new(args, item.clone());
    logic::override_trait_methods(&mut dax);

    dax.output_result(item)
}
