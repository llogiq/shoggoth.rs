use bit::{
    _0,
    _1,
    Bit,
};
use nat::{
    IsNat,
    Pos,
    W,
};
use nat::ops::*;

// Fn: Pred ////////////////////////////////////////////////////////////////////

// unwrap/rewrap
impl<N: IsNat, Rec: IsNat> Fn<(W<N>,)> for Pred where
    Pred: Fn(N) -> Rec
{
    type Output = W<Rec>;
    #[inline]
    extern "rust-call" fn call(&self, (W(n),): (W<N>,)) -> W<Rec> {
        W(Pred.call((n,)))
    }
}

/// `pred(b) ==> 0`
impl<B: Bit> Fn<(B,)> for Pred {
    type Output = _0;
    #[inline]
    extern "rust-call" fn call(&self, (_b,): (B,)) -> _0 {
        _0
    }
}
/// `pred(p:0) ==> pred_double(p)`
impl<P: Pos, Rec> Fn<((P, _0),)> for Pred where
    PredDouble: Fn(P) -> Rec
{
    type Output = Rec;
    #[inline]
    extern "rust-call" fn call(&self, ((p, _0 {}),): ((P, _0),)) -> Rec {
        PredDouble(p)
    }
}
/// `pred(p:1) ==> p:0`
impl<P: Pos> Fn<((P, _1),)> for Pred {
    type Output = (P, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((p, _1 {}),): ((P, _1),)) -> (P, _0) {
        (p, _0)
    }
}
