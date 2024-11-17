use override_macro::{overridable, override_with};

#[allow(dead_code)]
#[overridable]
trait BaseTrait05 {
    fn method1(&self) -> bool;
    fn method2(&self) -> u64;
}

#[allow(dead_code)]
#[overridable]
trait PartTrait05A {
    fn method1(&self) -> bool {
        true
    }
}

#[allow(dead_code)]
#[overridable]
trait PartTrait05B {
    fn method2(&self) -> u64 {
        987
    }
}

#[allow(dead_code)]
#[override_with(PartTrait05A, PartTrait05B)]
impl<T05: PartTrait05A + PartTrait05B> BaseTrait05 for T05 {}

#[allow(dead_code)]
struct Struct05;
impl PartTrait05A for Struct05 {}
impl PartTrait05B for Struct05 {}

#[allow(dead_code)]
fn exec(i: impl BaseTrait05) {
    assert!(i.method1());
    assert_eq!(i.method2(), 987);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = Struct05 {};
        exec(s);
    }
}
