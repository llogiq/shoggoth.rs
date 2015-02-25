use bit::{
    _0,
};
use nat::{
    IsNat,
    Pos,
    W,
};
use nat::ops::*;
use order;

// Fn: Compare /////////////////////////////////////////////////////////////////

impl<LHS: IsNat, RHS: IsNat, Rec> Fn<(W<LHS>, W<RHS>)> for Compare where
    Compare: Fn(LHS, RHS) -> Rec
{
    type Output = Rec;
    #[inline]
    extern "rust-call" fn call(&self, (W(lhs), W(rhs)): (W<LHS>, W<RHS>)) -> Rec {
        Compare(lhs, rhs)
    }
}

impl Fn<(_0, _0)> for Compare {
    type Output = order::Eq;
    #[inline]
    extern "rust-call" fn call(&self, (_0 {}, _0 {}): (_0, _0)) -> order::Eq {
        order::Eq
    }
}
impl<RHS: Pos> Fn<(_0, RHS)> for Compare {
    type Output = order::LT;
    #[inline]
    extern "rust-call" fn call(&self, (_0 {}, _rhs): (_0, RHS)) -> order::LT {
        order::LT
    }
}
impl<LHS: Pos> Fn<(LHS, _0)> for Compare {
    type Output = order::GT;
    #[inline]
    extern "rust-call" fn call(&self, (_lhs, _0 {}): (LHS, _0)) -> order::GT {
        order::GT
    }
}
impl<LHS: Pos, RHS: Pos, Rec> Fn<(LHS, RHS)> for Compare where
    CompareCont: Fn(LHS, RHS, order::Eq) -> Rec
{
    type Output = Rec;
    #[inline]
    extern "rust-call" fn call(&self, (lhs, rhs): (LHS, RHS)) -> Rec {
        CompareCont(lhs, rhs, order::Eq)
    }
}
