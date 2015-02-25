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
ty! { #[inline]
    fam Pred(W<N>,) => W<Rec> {
        Pred(W(m),) => W(Pred(m))
    } let {
        Rec = Pred(N,),
    } for :[ N: IsNat, Rec: IsNat ]
}

/// `pred(b) ==> 0`
ty! { #[inline]
    fam Pred(B,) => _0 {
        Pred(_,) => _0
    } for :[ B: Bit ]
}
/// `pred(p:0) ==> pred_double(p)`
ty! { #[inline]
    fam Pred((P, _0  ),) => Rec {
        Pred((p, _0{}),) => PredDouble(p)
    } let {
        Rec = PredDouble(P,),
    } for .[ Rec ] :[ P: Pos ]
}
/// `pred(p:1) ==> p:0`
ty! { #[inline]
    fam Pred((P, _1  ),) => (P, _0) {
        Pred((p, _1{}),) => (p, _0)
    } for :[ P: Pos ]
}
