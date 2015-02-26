use numerics::bit::*;
use numerics::nat::*;
use numerics::nat::ops::*;

// Fn: AddCarry ////////////////////////////////////////////////////////////////

/// `add_carry(1, 1) ==> 1:1`
ty! { #[inline]
    fam AddCarry(_1  , _1  ) => (_1, _1) {
        AddCarry(_1{}, _1{}) => (_1, _1)
    }
}
/// `add_carry(1, q:0) ==> succ(q):0`
ty! { #[inline]
    fam AddCarry(_1  , (Q, _0  )) => (Rec    , _0) {
        AddCarry(_1{}, (q, _0{})) => (Succ(q), _0)
    } let {
        Rec = Succ(Q,),
    } for .[ Rec ] :[ Q: Pos ]
}
/// `add_carry(1, q:1) ==> succ(q):1`
ty! { #[inline]
    fam AddCarry(_1  , (Q, _1  )) => (Rec    , _1) {
        AddCarry(_1{}, (q, _1{})) => (Succ(q), _1)
    } let {
        Rec = Succ(Q,),
    } for .[ Rec ] :[ Q: Pos ]
}
/// `add_carry(p:0, 1) ==> succ(p):0`
ty! { #[inline]
    fam AddCarry((P, _0  ), _1  ) => (Rec    , _0) {
        AddCarry((p, _0{}), _1{}) => (Succ(p), _0)
    } let {
        Rec = Succ(P,),
    } for .[ Rec ] :[ P: Pos ]
}
/// `add_carry(p:0, q:0) ==> add(p, q):1`
ty! { #[inline]
    fam AddCarry((P, _0  ), (Q, _0  )) => (Rec      , _1) {
        AddCarry((p, _0{}), (q, _0{})) => (Add(p, q), _1)
    } let {
        Rec = Add(P, Q),
    } for .[ Rec ] :[ P: Pos, Q: Pos ]
}
/// `add_carry(p:0, q:1) ==> add_carry(p, q):0`
ty! { #[inline]
    fam AddCarry((P, _0  ), (Q, _1  )) => (Rec           , _0) {
        AddCarry((p, _0{}), (q, _1{})) => (AddCarry(p, q), _0)
    } let {
        Rec = AddCarry(P, Q),
    } for .[ Rec ] :[ P: Pos, Q: Pos ]
}
/// `add_carry(p:1, 1) ==> succ(p):1`
ty! { #[inline]
    fam AddCarry((P, _1  ), _1  ) => (Rec    , _1) {
        AddCarry((p, _1{}), _1{}) => (Succ(p), _1)
    } let {
        Rec = Succ(P,),
    } for .[ Rec ] :[ P: Pos ]
}
/// `add_carry(p:1, q:0) ==> add_carry(p, q):0`
ty! { #[inline]
    fam AddCarry((P, _1  ), (Q, _0  )) => (Rec           , _0) {
        AddCarry((p, _1{}), (q, _0{})) => (AddCarry(p, q), _0)
    } let {
        Rec = AddCarry(P, Q),
    } for .[ Rec ] :[ P: Pos, Q: Pos ]
}
/// `add_carry(p:1, q:1) ==> add_carry(p, q):1`
ty! { #[inline]
    fam AddCarry((P, _1  ), (Q, _1  )) => (Rec           , _1) {
        AddCarry((p, _1{}), (q, _1{})) => (AddCarry(p, q), _1)
    } let {
        Rec = AddCarry(P, Q),
    } for .[ Rec ] :[ P: Pos, Q: Pos ]
}
