use bit::{
    _0,
    _1,
    Bit,
};
use std::marker::{
    PhantomFn,
};

mod boilerplate;

// Nat wrapper struct (grumble, grumble, coherence...)
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct W<N: IsNat>(N);

// Classify valid binary nats (positive)
pub trait Pos: PhantomFn<Self> + IsNat {}
impl Pos for _1 {}
impl<P: Pos, B: Bit> Pos for (P, B) {}

pub trait IsNat: PhantomFn<Self> {}
impl IsNat for _0 {}
impl<P: Pos> IsNat for P {}

// Classify valid binary nats (with zero)
pub trait Nat: PhantomFn<Self> {}
impl<N: IsNat> Nat for W<N> {}

pub struct Add;
pub struct AddCarry;
pub struct Succ;

#[cfg(test)]
mod test {
    use bit;
    use nat;

    #[test]
    fn add() {
        let _: Nat!(32768) = nat!(16384) + nat!(16384);
    }
}
