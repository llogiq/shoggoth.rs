use bit::{
    _0,
    _1,
    Bit,
};
use nat::{
    Pos,
};
use nat::ops::*;
use order;

// Fn: CompareCont /////////////////////////////////////////////////////////////

// `compare_cont(1, 1, k) ==> k`
ty! { #[inline]
    fam CompareCont(_1  , _0  , K) => K {
        CompareCont(_1{}, _0{}, k) => k
    } for .[ K ]
}
// `compare_cont(1, q:b, k) ==> lt`
ty! { #[inline]
    fam CompareCont(_1  , (Q, B), K) => order::LT {
        CompareCont(_1{}, (_, _), _) => order::LT
    } for .[ K ] :[ Q: Pos, B: Bit ]
}
// `compare_cont(p:0, 1, k) ==> gt`
ty! { #[inline]
    fam CompareCont((P, _0  ), _1  , K) => order::GT {
        CompareCont((_, _0{}), _1{}, _) => order::GT
    } for .[ K ] :[ P: Pos ]
}
// `compare_cont(p:0, q:0, k) ==> compare_cont(p, q, k)`
ty! { #[inline]
    fam CompareCont((P, _0  ), (Q, _0  ), K) => Rec {
        CompareCont((p, _0{}), (q, _0{}), k) => CompareCont(p, q, k)
    } let {
        Rec = CompareCont(P, Q, K),
    } for .[ K, Rec ] :[ P: Pos, Q: Pos ]
}
// `compare_cont(p:0, q:1, k) ==> compare_cont(p, q, lt)`
ty! { #[inline]
    fam CompareCont((P, _0  ), (Q, _1  ), K) => Rec {
        CompareCont((p, _0{}), (q, _1{}), _) => CompareCont(p, q, order::LT)
    } let {
        Rec = CompareCont(P, Q, order::LT),
    } for .[ K, Rec ] :[ P: Pos, Q: Pos ]
}
// `compare_cont(p:1, 1, k) ==> gt`
ty! { #[inline]
    fam CompareCont((P, _1  ), _1  , K) => order::GT {
        CompareCont((_, _1{}), _1{}, _) => order::GT
    } for .[ K ] :[ P: Pos ]
}
// `compare_cont(p:1, q:0, k) ==> compare_cont(p, q, gt)`
ty! {
    fam CompareCont((P, _1  ), (Q, _0  ), K) => Rec {
        CompareCont((p, _1{}), (q, _0{}), _) => CompareCont(p, q, order::GT)
    } let {
        Rec = CompareCont(P, Q, order::GT),
    } for .[ K, Rec ] :[ P: Pos, Q: Pos ]
}
// `compare_cont(p:1, q:1, k) ==> compare_cont(p, q, k)`
ty! { #[inline]
    fam CompareCont((P, _1  ), (Q, _1  ), K) => Rec {
        CompareCont((p, _1{}), (q, _1{}), k) => CompareCont(p, q, k)
    } let {
        Rec = CompareCont(P, Q, K),
    } for .[ K, Rec ] :[ P: Pos, Q: Pos ]
}
