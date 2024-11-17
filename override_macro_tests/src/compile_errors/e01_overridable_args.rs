use override_macro::overridable;

#[overridable(a)]
trait Trait01A {}

#[overridable(mod a)]
trait Trait01B {}

#[overridable(mod =)]
trait Trait01C {}

#[overridable(mod = a())]
trait Trait01D {}

#[overridable(mod = a)]
trait Trait01E {}

fn main() {
}
