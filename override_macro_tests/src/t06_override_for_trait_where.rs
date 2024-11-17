use override_macro::{overridable, override_with};

#[allow(dead_code)]
#[overridable]
trait BaseTrait06 {
    fn method1(&self) -> bool;
    fn method2(&self) -> u64;
}

#[allow(dead_code)]
#[overridable]
trait PartTrait06A {
    fn method1(&self) -> bool {
        true
    }
}

#[allow(dead_code)]
#[overridable]
trait PartTrait06B {
    fn method2(&self) -> u64 {
        987
    }
}

#[allow(dead_code)]
#[override_with(PartTrait06A, PartTrait06B)]
impl<T06> BaseTrait06 for T06 where T06: PartTrait06A + PartTrait06B {}

#[allow(dead_code)]
struct Struct06;
impl PartTrait06A for Struct06 {}
impl PartTrait06B for Struct06 {}

#[allow(dead_code)]
fn exec(i: impl BaseTrait06) {
    assert!(i.method1());
    assert_eq!(i.method2(), 987);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = Struct06 {};
        exec(s);
    }
}
