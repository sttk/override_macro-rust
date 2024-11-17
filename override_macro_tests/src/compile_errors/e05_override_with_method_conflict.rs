use override_macro::{overridable, override_with};

#[overridable]
trait Trait05A {
  fn method0(&self) -> bool;
}

#[overridable]
trait Trait05B {
  fn method0(&self) -> bool {
    true
  }
}

#[overridable]
trait Trait05C {
  fn method0(&self) -> bool {
    false
  }
}

struct Struct05;
impl Trait05B for Struct05 {}
impl Trait05C for Struct05 {}

#[override_with(Trait05B, Trait05C)]
impl Trait05A for Struct05 {}

fn main() {
}
