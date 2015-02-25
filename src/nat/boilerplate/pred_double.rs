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
ty! { #[inline]
    fam PredDouble(_1  ,) => _1 {
        PredDouble(_1{},) => _1
    }
}
/// `pred_double(p:0) ==> pred_double(p):1`
ty! { #[inline]
    fam PredDouble((P, _0)  ,) => (Rec          , _1) {
        PredDouble((p, _0{}),) => (PredDouble(p), _1)
    } let {
        Rec = PredDouble(P,),
    } for .[ Rec ] :[ P: Pos ]
}
/// `pred_double(p:1) ==> p:0:1`
ty! { #[inline]
    fam PredDouble((P, _1)  ,) => ((P, _0), _1) {
        PredDouble((p, _1{}),) => ((p, _0), _1)
    } for :[ P: Pos ]
}
