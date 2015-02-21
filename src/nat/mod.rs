use bit::{
    _0,
    _1,
    IsBit,
};
use std::marker::{
    PhantomFn,
};

mod boilerplate;

// Nat wrapper struct (grumble, grumble, coherence...)
pub struct Nat<N: IsNat>(N);

// Classify valid binary nats (positive)
pub trait IsPos: PhantomFn<Self> + IsNat {}
impl IsPos for _1 {}
impl<P: IsPos, B: IsBit> IsPos for (P, B) {}

// Classify valid binary nats (with zero)
pub trait IsNat: PhantomFn<Self> {}
impl IsNat for _0 {}
impl IsNat for _1 {}
impl<P: IsPos, B: IsBit> IsNat for (P, B) {}

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
