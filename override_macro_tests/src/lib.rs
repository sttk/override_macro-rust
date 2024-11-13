use override_macro::my_attr_macro;

#[my_attr_macro]
pub trait Trait0 {
    fn method00(&self) -> bool;
}

#[derive(Debug)]
pub struct Struct0 {}

impl Trait0 for Struct0 {
    fn method00(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s0 = Struct0 {};
        assert!(s0.method00());
    }
}
