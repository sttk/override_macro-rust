use override_macro::{overridable, override_with};

#[allow(dead_code)]
#[overridable]
trait BaseTrait09 {
    fn method0(&self) -> bool;
    fn method1(&self, b: bool) -> u64;
}

#[allow(dead_code)]
#[overridable]
trait DefaultMethodTrait09A {
    fn method0(&self) -> bool {
        true
    }
    fn method1(&self, _b: bool) -> u64 {
        123
    }
}

#[allow(dead_code)]
#[overridable]
trait DefaultMethodTrait09B {
    fn method1(&self, _b: bool) -> u64 {
        987
    }
}

#[allow(dead_code)]
struct Struct09;
impl DefaultMethodTrait09A for Struct09 {}
impl DefaultMethodTrait09B for Struct09 {}

#[override_with(DefaultMethodTrait09A, DefaultMethodTrait09B)]
impl BaseTrait09 for Struct09 {
    fn method1(&self, b: bool) -> u64 {
        DefaultMethodTrait09B::method1(self, b)
    }
}

#[allow(dead_code)]
fn exec(i: impl BaseTrait09) {
    assert!(i.method0());
    assert_eq!(i.method1(true), 987);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = Struct09 {};
        exec(s);
    }
}
