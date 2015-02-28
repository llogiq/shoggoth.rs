use bit::*;
use numerics::nat::bin::*;
use numerics::nat::bin::ops::*;

// Fn: Pred ////////////////////////////////////////////////////////////////////

// unwrap/rewrap
ty! {
    fam :[ Pred(W<N>,) => W<Rec> ]
        =[ Pred(W(m),) => W(Pred(m)) ]
    let {
        Rec = Pred(N,),
    } for { N, Rec } where { N: IsNat, Rec: IsNat }
}

/// `pred(b) ==> 0`
ty! {
    fam :[ Pred(B,) => _0 ]
        =[ Pred(_,) => _0 ]
    for { B } where { B: Bit }
}
/// `pred(p:0) ==> pred_double(p)`
ty! {
    fam :[ Pred((P, _0  ),) => Rec ]
        =[ Pred((p, _0{}),) => PredDouble(p) ]
    let {
        Rec = PredDouble(P,),
    } for { P, Rec } where { P: Pos }
}
/// `pred(p:1) ==> p:0`
ty! {
    fam :[ Pred((P, _1  ),) => (P, _0) ]
        =[ Pred((p, _1{}),) => (p, _0) ]
    for { P } where { P: Pos }
}
