use bit::*;
use numerics::nat::bin::*;
use numerics::nat::bin::ops::*;
use order;

// Fn: CompareCont /////////////////////////////////////////////////////////////

// `compare_cont(1, 1, k) ==> k`
ty! {
    fam :[ CompareCont(_1  , _0  , K) => K ]
        =[ CompareCont(_1{}, _0{}, k) => k ]
    for { K }
}
// `compare_cont(1, q:b, k) ==> lt`
ty! {
    fam :[ CompareCont(_1  , (Q, B), K) => order::LT ]
        =[ CompareCont(_1{}, (_, _), _) => order::LT ]
    for { Q, B, K } where { Q: Pos, B: Bit }
}
// `compare_cont(p:0, 1, k) ==> gt`
ty! {
    fam :[ CompareCont((P, _0  ), _1  , K) => order::GT ]
        =[ CompareCont((_, _0{}), _1{}, _) => order::GT ]
    for { P, K } where { P: Pos }
}
// `compare_cont(p:0, q:0, k) ==> compare_cont(p, q, k)`
ty! {
    fam :[ CompareCont((P, _0  ), (Q, _0  ), K) => Rec ]
        =[ CompareCont((p, _0{}), (q, _0{}), k) => CompareCont(p, q, k) ]
    let {
        Rec = CompareCont(P, Q, K),
    } for { P, Q, K, Rec } where { P: Pos, Q: Pos }
}
// `compare_cont(p:0, q:1, k) ==> compare_cont(p, q, lt)`
ty! {
    fam :[ CompareCont((P, _0  ), (Q, _1  ), K) => Rec ]
        =[ CompareCont((p, _0{}), (q, _1{}), _) => CompareCont(p, q, order::LT) ]
    let {
        Rec = CompareCont(P, Q, order::LT),
    } for { P, Q, K, Rec } where { P: Pos, Q: Pos }
}
// `compare_cont(p:1, 1, k) ==> gt`
ty! {
    fam :[ CompareCont((P, _1  ), _1  , K) => order::GT ]
        =[ CompareCont((_, _1{}), _1{}, _) => order::GT ]
    for { P, K } where { P: Pos }
}
// `compare_cont(p:1, q:0, k) ==> compare_cont(p, q, gt)`
ty! {
    fam :[ CompareCont((P, _1  ), (Q, _0  ), K) => Rec ]
        =[ CompareCont((p, _1{}), (q, _0{}), _) => CompareCont(p, q, order::GT) ]
    let {
        Rec = CompareCont(P, Q, order::GT),
    } for { P, Q, K, Rec } where { P: Pos, Q: Pos }
}
// `compare_cont(p:1, q:1, k) ==> compare_cont(p, q, k)`
ty! {
    fam :[ CompareCont((P, _1  ), (Q, _1  ), K) => Rec ]
        =[ CompareCont((p, _1{}), (q, _1{}), k) => CompareCont(p, q, k) ]
    let {
        Rec = CompareCont(P, Q, K),
    } for { P, Q, K, Rec } where { P: Pos, Q: Pos }
}