use bit::*;
use std::marker::{
    PhantomData,
    PhantomFn,
};

pub mod pos {
    use bit::*;
    use std::marker::PhantomFn;

    pub trait Pos: PhantomFn<Self> {}
    impl Pos for _1 {}
    impl<P: Pos, B> Pos for (P, B) {}
}

pub trait Nat: PhantomFn<Self> {}
impl Nat for _0 {}
impl Nat for _1 {}
impl<P: pos::Pos, B> Nat for (P, B) {}

#[cfg(test)]
mod test {
    use bit;

    #[test]
    fn welp() {
        let _: Nat!(42) = nat!(42);
    }
}
