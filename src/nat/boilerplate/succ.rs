use bit::{
    _0,
    _1,
};
use nat::{
    IsNat,
    Pos,
    W,
};
use nat::ops::*;

// Fn: Succ ////////////////////////////////////////////////////////////////////

// unwrap/rewrap
impl<N: IsNat, Rec: IsNat> Fn<(W<N>,)> for Succ where
    Succ: Fn(N) -> Rec
{
    type Output = W<Rec>;
    #[inline]
    extern "rust-call" fn call(&self, (W(n),): (W<N>,)) -> W<Rec> {
        W(Succ.call((n,)))
    }
}

/// `succ(0) ==> 1`
impl Fn<(_0,)> for Succ {
    type Output = _1;
    #[inline]
    extern "rust-call" fn call(&self, (_0 {},): (_0,)) -> _1 {
        _1
    }
}
/// `succ(1) ==> 1:0`
impl Fn<(_1,)> for Succ {
    type Output = (_1, _0);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {},): (_1,)) -> (_1, _0) {
        (_1, _0)
    }
}
/// `succ(p:0) ==> p:1`
impl<LHS: Pos> Fn<((LHS, _0),)> for Succ {
    type Output = (LHS, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _),): ((LHS, _0),)) -> (LHS, _1) {
        (lhs, _1)
    }
}
/// `succ(p:1) ==> succ(p):0`
impl<LHS: Pos, Rec> Fn<((LHS, _1),)> for Succ where
    Succ: Fn(LHS) -> Rec
{
    type Output = (Rec, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _),): ((LHS, _1),)) -> (Rec, _0) {
        (Succ(lhs), _0)
    }
}
