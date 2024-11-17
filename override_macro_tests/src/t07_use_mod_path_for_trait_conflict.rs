use override_macro::{overridable, override_with};

#[allow(dead_code)]
#[overridable]
trait BaseTrait07 {
    fn method0(&self) -> u64;
    fn method1(&self) -> String;
}

#[allow(dead_code)]
#[overridable(mod = crate::t07_use_mod_path_for_trait_conflict)]
trait Trait07A {
    fn method0(&self) -> u64 {
        123
    }
}

mod module07_a {
    use super::*;

    #[allow(dead_code)]
    #[overridable(mod = crate::t07_use_mod_path_for_trait_conflict::module07_a)]
    pub trait Trait07A {
        fn method2(&self) -> u64 {
            987
        }
    }

    #[allow(dead_code)]
    #[overridable(mod = crate::t07_use_mod_path_for_trait_conflict::module07_a)]
    pub trait Trait07B {
        fn method1(&self) -> String {
            "module07_a::method1".to_string()
        }
    }

    pub mod module07_b {
        use super::*;

        #[allow(dead_code)]
        #[overridable(mod = crate::t07_use_mod_path_for_trait_conflict::module07_a::module07_b)]
        pub trait Trait07B {
            fn method4(&self) -> String {
                "module07_b::method1".to_string()
            }
        }
    }
}

#[allow(dead_code)]
struct Struct07;
impl Trait07A for Struct07 {}
impl module07_a::Trait07B for Struct07 {}

#[override_with(
    crate::t07_use_mod_path_for_trait_conflict::Trait07A,
    crate::t07_use_mod_path_for_trait_conflict::module07_a::Trait07B
)]
impl BaseTrait07 for Struct07 {}

#[allow(dead_code)]
fn exec(i: impl BaseTrait07) {
    assert_eq!(i.method0(), 123);
    assert_eq!(i.method1(), "module07_a::method1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = Struct07 {};
        exec(s);
    }
}
