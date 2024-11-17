use override_macro::{overridable, override_with};

#[allow(dead_code)]
#[overridable]
trait BaseTrait01 {
    fn method0(&self) -> bool;
    fn method1(&self, b: bool) -> u64;
}

#[allow(dead_code)]
#[overridable]
trait DefaultMethodTrait01A {
    fn method0(&self) -> bool {
        true
    }
}

#[allow(dead_code)]
#[overridable]
trait DefaultMethodTrait01B {
    fn method1(&self, _b: bool) -> u64 {
        123
    }
}

#[allow(dead_code)]
struct Struct01;
impl DefaultMethodTrait01A for Struct01 {}
impl DefaultMethodTrait01B for Struct01 {}

#[override_with(DefaultMethodTrait01A, DefaultMethodTrait01B)]
impl BaseTrait01 for Struct01 {}

#[allow(dead_code)]
fn exec(i: impl BaseTrait01) {
    assert!(i.method0());
    assert_eq!(i.method1(true), 123);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = Struct01 {};
        exec(s);
    }
}
