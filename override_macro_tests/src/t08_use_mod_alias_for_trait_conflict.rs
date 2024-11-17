use crate::t08_use_mod_alias_for_trait_conflict as t08;
use override_macro::{overridable, override_with};

#[allow(dead_code)]
#[overridable]
trait BaseTrait08 {
    fn method0(&self) -> u64;
    fn method1(&self) -> String;
}

#[allow(dead_code)]
#[overridable(mod = t08)]
trait Trait08A {
    fn method0(&self) -> u64 {
        123
    }
}

mod module08_a {
    use super::*;

    #[allow(dead_code)]
    #[overridable(mod = t08::module08_a)]
    pub trait Trait08A {
        fn method2(&self) -> u64 {
            987
        }
    }

    #[allow(dead_code)]
    #[overridable(mod = t08::module08_a)]
    pub trait Trait08B {
        fn method1(&self) -> String {
            "module08_a::method1".to_string()
        }
    }

    pub mod module08_b {
        use super::*;

        #[allow(dead_code)]
        #[overridable(mod = t08::module08_a::module08_b)]
        pub trait Trait08B {
            fn method4(&self) -> String {
                "module08_b::method1".to_string()
            }
        }
    }
}

#[allow(dead_code)]
struct Struct08;
impl Trait08A for Struct08 {}
impl module08_a::Trait08B for Struct08 {}

#[override_with(t08::Trait08A, t08::module08_a::Trait08B)]
impl BaseTrait08 for Struct08 {}

#[allow(dead_code)]
fn exec(i: impl BaseTrait08) {
    assert_eq!(i.method0(), 123);
    assert_eq!(i.method1(), "module08_a::method1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = Struct08 {};
        exec(s);
    }
}
