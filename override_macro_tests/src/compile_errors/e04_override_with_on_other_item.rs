use override_macro::{overridable, override_with};

#[overridable]
trait Trait04A {}

#[override_with(Trait04A)]
trait Trait04B {}

fn main() {
}
