use override_macro::{overridable, override_with};

#[overridable]
trait Trait07 {
  fn method0(&self) -> bool;
}

struct Struct07;

#[override_with(Trait07)]
impl Trait07 for Struct07 {}

fn main() {
  let _ = Struct07{};
}
