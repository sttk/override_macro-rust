use override_macro::{overridable, override_with};

#[overridable]
trait Trait06A {}

#[overridable]
trait Trait06B {}

#[overridable]
trait Trait06C {}

mod module_a {
  use super::*;

  #[overridable]
  trait Trait06B {}

  #[overridable]
  trait Trait06C {}
}

struct Struct06A;
impl Trait06B for Struct06A {}

#[override_with(Trait06B)]
impl Trait06A for Struct06A {} 

struct Struct06B;
impl Trait06A for Struct06B {}

#[override_with(Trait06A)]
impl Trait06C for Struct06B {} 

fn main() {
}
