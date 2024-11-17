use override_macro::{overridable, override_with};

#[overridable]
trait Trait03A {}

#[overridable]
trait Trait03B {}

struct Struct03A;
impl Trait03A for Struct03A {}

#[override_with()]
impl Trait03B for Struct03A {}

struct Struct03B;
impl Trait03A for Struct03B {}

#[override_with(a())]
impl Trait03B for Struct03B {}

fn main() {
}
