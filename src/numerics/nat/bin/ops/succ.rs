use bit::*;
use numerics::nat::bin::*;
use numerics::nat::bin::ops::*;

// Fn: Succ ////////////////////////////////////////////////////////////////////

// unwrap/rewrap
ty! {
    fam Succ(W<N>,) => W<Rec> {
        Succ(W(m),) => W(Succ(m))
    } let {
        Rec = Succ(N,),
    } for :[ N: IsNat, Rec: IsNat ]
}

/// `succ(0) ==> 1`
ty! {
    fam Succ(_0  ,) => _1 {
        Succ(_0{},) => _1
    }
}
/// `succ(1) ==> 1:0`
ty! {
    fam Succ(_1  ,) => (_1, _0) {
        Succ(_1{},) => (_1, _0)
    }
}
/// `succ(p:0) ==> p:1`
ty! {
    fam Succ((P, _0  ),) => (P, _1) {
        Succ((p, _0{}),) => (p, _1)
    } for :[ P: Pos ]
}
/// `succ(p:1) ==> succ(p):0`
ty! {
    fam Succ((P, _1  ),) => (Rec    , _0) {
        Succ((p, _1{}),) => (Succ(p), _0)
    } let {
        Rec = Succ(P,),
    } for .[ Rec ] :[ P: Pos ]
}
