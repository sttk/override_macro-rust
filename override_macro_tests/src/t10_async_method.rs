#![allow(dead_code)]

use override_macro::{overridable, override_with};

#[overridable]
trait BaseTrait10 {
    async fn method0(&self) -> bool;
    async fn method1(&mut self, b: bool) -> u64;
}

#[overridable]
trait DefaultMethodTrait10A {
    async fn method0(&self) -> bool {
        true
    }
}

#[overridable]
trait DefaultMethodTrait10B {
    async fn method1(&mut self, _b: bool) -> u64 {
        123
    }
}

struct Struct10;
impl DefaultMethodTrait10A for Struct10 {}
impl DefaultMethodTrait10B for Struct10 {}

#[override_with(DefaultMethodTrait10A, DefaultMethodTrait10B)]
impl BaseTrait10 for Struct10 {}
/*
    async fn method0(&self) -> bool {
        DefaultMethodTrait10A::method0(self).await
    }
    async fn method1(&mut self, _b: bool) -> u64 {
        DefaultMethodTrait10B::method1(self, _b).await
    }
*/

async fn exec(mut i: impl BaseTrait10) {
    assert!(i.method0().await);
    assert_eq!(i.method1(true).await, 123);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        let s = Struct10 {};
        exec(s).await;
    }
}
