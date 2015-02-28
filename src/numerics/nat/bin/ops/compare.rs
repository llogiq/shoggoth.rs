use bit::*;
use numerics::nat::bin::*;
use numerics::nat::bin::ops::*;
use order;

// Fn: Compare /////////////////////////////////////////////////////////////////

// unwrap
ty! {
    fam :[ Compare(W<M>, W<N>) => Rec ]
        =[ Compare(W(m), W(n)) => Compare(m, n) ]
    let {
        Rec = Compare(M, N),
    } for { M, N, Rec } where { M: IsNat, N: IsNat }
}

/// compare(0, 0) ==> eq
ty! {
    fam :[ Compare(_0  , _0  ) => order::Eq ]
        =[ Compare(_0{}, _0{}) => order::Eq ]
}
/// compare(0, q) ==> lt
ty! {
    fam :[ Compare(_0  , Q) => order::LT ]
        =[ Compare(_0{}, _) => order::LT ]
    for { Q } where { Q: Pos }
}
/// compare(p, 0) ==> gt
ty! {
    fam :[ Compare(P, _0  ) => order::GT ]
        =[ Compare(_, _0{}) => order::GT ]
    for { P } where { P: Pos }
}
/// compare(p, q) ==> compare_cont(p, q, eq)
ty! {
    fam :[ Compare(P, Q) => Rec ]
        =[ Compare(p, q) => CompareCont(p, q, order::Eq) ]
    let {
        Rec = CompareCont(P, Q, order::Eq),
    } for { P, Q, Rec } where { P: Pos, Q: Pos }
}
