use override_macro::overridable;

#[overridable]
trait Trait02A {}

struct Struct02;

#[overridable]
impl Trait02A for Struct02 {}

fn main() {
}
