use bit::{
    _0,
    _1,
};
use nat::{
    Pos,
};
use nat::ops::*;

// Fn: PredDouble //////////////////////////////////////////////////////////////

/// `pred_double(_1) ==> _1`
impl Fn<(_1,)> for PredDouble {
    type Output = _1;
    #[inline]
    extern "rust-call" fn call(&self, (_1 {},): (_1,)) -> _1 {
        _1
    }
}
/// `pred_double(p:0) ==> pred_double(p):1`
impl<P: Pos, Rec> Fn<((P, _0),)> for PredDouble where
    PredDouble: Fn(P) -> Rec
{
    type Output = (Rec, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((p, _0 {}),): ((P, _0),)) -> (Rec, _1) {
        (PredDouble(p), _1)
    }
}
/// `pred_double(p:1) ==> p:0:1`
impl<P: Pos> Fn<((P, _1),)> for PredDouble {
    type Output = ((P, _0), _1);
    #[inline]
    extern "rust-call" fn call(&self, ((p, _1 {}),): ((P, _1),)) -> ((P, _0), _1) {
        ((p, _0), _1)
    }
}
