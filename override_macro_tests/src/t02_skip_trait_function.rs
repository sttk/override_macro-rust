use override_macro::{overridable, override_with};

#[allow(dead_code)]
#[overridable]
trait BaseTrait02 {
    fn function0() -> String; // skipped overriding
    fn method0(&self) -> bool;
}

#[allow(dead_code)]
#[overridable]
trait DefaultMethodTrait02 {
    fn function0() -> String {
        "hello".to_string()
    }
    fn method0(&self) -> bool {
        true
    }
}

#[allow(dead_code)]
struct Struct02;
impl DefaultMethodTrait02 for Struct02 {}

#[override_with(DefaultMethodTrait02)]
impl BaseTrait02 for Struct02 {
    // needed this implementation because function is not overridden
    fn function0() -> String {
        "function0".to_string()
    }
}

#[allow(dead_code)]
fn exec(i: impl BaseTrait02) {
    assert!(i.method0());
    assert_eq!(<Struct02 as BaseTrait02>::function0(), "function0");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = Struct02 {};
        exec(s);
    }
}
