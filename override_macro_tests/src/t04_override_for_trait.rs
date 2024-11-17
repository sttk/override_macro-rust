use override_macro::{overridable, override_with};

#[allow(dead_code)]
#[overridable]
trait BaseTrait04 {
    fn method1(&self) -> bool;
    fn method2(&self) -> u64;
}

#[allow(dead_code)]
#[overridable]
trait PartTrait04A {
    fn method1(&self) -> bool {
        true
    }
}

#[allow(dead_code)]
#[overridable]
trait PartTrait04B {
    fn method2(&self) -> u64 {
        987
    }
}

trait CombinedTrait04: PartTrait04A + PartTrait04B {}

#[allow(dead_code)]
#[override_with(PartTrait04A, PartTrait04B)]
impl<T04: CombinedTrait04> BaseTrait04 for T04 {}

#[allow(dead_code)]
struct Struct04;
impl PartTrait04A for Struct04 {}
impl PartTrait04B for Struct04 {}
impl CombinedTrait04 for Struct04 {}

#[allow(dead_code)]
fn exec(i: impl BaseTrait04) {
    assert!(i.method1());
    assert_eq!(i.method2(), 987);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = Struct04 {};
        exec(s);
    }
}
