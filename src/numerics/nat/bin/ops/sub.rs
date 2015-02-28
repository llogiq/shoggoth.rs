use bit::*;
use numerics::nat::bin::*;
use numerics::nat::bin::ops::*;
use std;

// Fn: Sub /////////////////////////////////////////////////////////////////////

// unwrap/rewrap
ty! {
    fam Sub(W<M>, W<N>) => W<Rec> {
        Sub(W(m), W(n)) => W(Sub(m, n))
    } let {
        Rec = Sub(M, N),
    } for :[ M: IsNat, N: IsNat, Rec: IsNat ]
}

/// `sub(0, 0) ==> 0`
ty! {
    fam Sub(_0  , _0  ) => _0 {
        Sub(_0{}, _0{}) => _0
    }
}
/// `sub(0, q) ==> 0`
ty! {
    fam Sub(_0  , Q) => _0 {
        Sub(_0{}, _) => _0
    } for :[ Q: Pos ]
}
/// `sub(p, 0) ==> p`
ty! {
    fam Sub(P, _0  ) => P {
        Sub(p, _0{}) => p
    } for :[ P: Pos ]
}
/// `sub(p, q) ==> sub_cont(sub_mask(p, q))
ty! {
    fam Sub(P, Q) => Rec1 {
        Sub(p, q) => SubCont(SubMask(p, q))
    } let {
        Rec0 = SubMask(P, Q),
        Rec1 = SubCont(Rec0,),
    } for .[ Rec0, Rec1 ] :[ P: Pos, Q: Pos ]
}

// Infix: Sub //////////////////////////////////////////////////////////////////

impl<M: IsNat, N: IsNat, Rec: IsNat> std::ops::Sub<W<N>> for W<M> where
    Sub: Fn(M, N) -> Rec
{
    type Output = W<Rec>;
    fn sub(self, rhs: W<N>) -> W<Rec> {
        W(Sub(self.0, rhs.0))
    }
}
