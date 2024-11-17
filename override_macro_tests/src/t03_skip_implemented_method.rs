use override_macro::{overridable, override_with};

#[allow(dead_code)]
#[overridable]
trait BaseTrait03 {
    fn method0(&self) -> String {
        "BaseTrait03::method0".to_string()
    }
    fn method1(&self, b: bool) -> u64;
}

#[allow(dead_code)]
#[overridable]
trait DefaultMethodTrait03 {
    fn method0(&self) -> String {
        "DefaultMethodTrait03::method0".to_string()
    }
    fn method1(&self, _b: bool) -> u64 {
        987
    }
}

#[allow(dead_code)]
struct Struct03;
impl DefaultMethodTrait03 for Struct03 {}

#[override_with(DefaultMethodTrait03)]
impl BaseTrait03 for Struct03 {}

#[allow(dead_code)]
fn exec(i: impl BaseTrait03) {
    assert_eq!(i.method0(), "BaseTrait03::method0");
    assert_eq!(i.method1(true), 987);
}

#[allow(dead_code)]
struct Struct03A;
impl DefaultMethodTrait03 for Struct03A {}

#[override_with(DefaultMethodTrait03)]
impl BaseTrait03 for Struct03A {
    fn method0(&self) -> String {
        DefaultMethodTrait03::method0(self)
    }
}

#[allow(dead_code)]
fn exec_a(i: impl BaseTrait03) {
    assert_eq!(i.method0(), "DefaultMethodTrait03::method0");
    assert_eq!(i.method1(true), 987);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = Struct03 {};
        exec(s);
    }

    #[test]
    fn test_a() {
        let s = Struct03A {};
        exec_a(s);
    }
}
