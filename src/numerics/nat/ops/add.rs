use bit::{
    _0,
    _1,
};
use nat::{
    IsNat,
    Pos,
    W,
};
use std;

// Fn: Add /////////////////////////////////////////////////////////////////////

// unwrap/rewrap
ty! { #[inline]
    fam Add(W<M>, W<N>) => W<Rec> {
        Add(W(m), W(n)) => W(Add(m, n))
    } let {
        Rec = Add(M, N),
    } for :[ M: IsNat, N: IsNat, Rec: IsNat ]
}

/// `add(0, 0) ==> 0`
ty! { #[inline]
    fam Add(_0  , _0  ) => _0 {
        Add(_0{}, _0{}) => _0
    }
}
/// `add(0, q) ==> q`
ty! { #[inline]
    fam Add(_0  , Q) => Q {
        Add(_0{}, q) => q
    } for :[ Q: Pos ]
}
/// `add(p, 0) ==> p`
ty! { #[inline]
    fam Add(P, _0  ) => P {
        Add(p, _0{}) => p
    } for :[ P: Pos ]
}
/// `add(1, 1) ==> 1:0`
ty! { #[inline]
    fam Add(_1  , _1  ) => (_1, _0) {
        Add(_1{}, _1{}) => (_1, _0)
    }
}
/// `add(1, q:0) ==> q:1`
ty! { #[inline]
    fam Add(_1  , (Q, _0  )) => (Q, _1) {
        Add(_1{}, (q, _0{})) => (q, _1)
    } for :[ Q: Pos ]
}
/// `add(1, q:1) ==> succ(q):0`
ty! { #[inline]
    fam Add(_1  , (Q, _1  )) => (Rec    , _0) {
        Add(_1{}, (q, _1{})) => (Succ(q), _0)
    } let {
        Rec = Succ(Q,),
    } for .[ Rec ] :[ Q: Pos ]
}
/// `add(p:0, 1) ==> p:1`
ty! { #[inline]
    fam Add((P, _0  ), _1  ) => (P, _1) {
        Add((p, _0{}), _1{}) => (p, _1)
    } for :[ P: Pos ]
}
/// `add(p:0, q:0) ==> add(p, q):0`
ty! { #[inline]
    fam Add((P, _0  ), (Q, _0  )) => (Rec      , _0) {
        Add((p, _0{}), (q, _0{})) => (Add(p, q), _0)
    } let {
        Rec = Add(P, Q),
    } for .[ Rec ] :[ P: Pos, Q: Pos ]
}
/// `add(p:0, q:1) ==> add(p, q):1`
ty! { #[inline]
    fam Add((P, _0  ), (Q, _1  )) => (Rec      , _1) {
        Add((p, _0{}), (q, _1{})) => (Add(p, q), _1)
    } let {
        Rec = Add(P, Q),
    } for .[ Rec ] :[ P: Pos, Q: Pos ]
}
/// `add(p:1, 1) ==> succ(p):0`
ty! { #[inline]
    fam Add((P, _1  ), _1  ) => (Rec    , _0) {
        Add((p, _1{}), _1{}) => (Succ(p), _0)
    } let {
        Rec = Succ(P,),
    } for .[ Rec ] :[ P: Pos ]
}
/// `add(p:1, q:0) ==> add(p, q):1`
ty! { #[inline]
    fam Add((P, _1  ), (Q, _0  )) => (Rec      , _1) {
        Add((p, _1{}), (q, _0{})) => (Add(p, q), _1)
    } let {
        Rec = Add(P, Q),
    } for .[ Rec ] :[ P: Pos, Q: Pos ]
}
/// `add(p:1, q:1) ==> add_carry(p, q):0`
ty! { #[inline]
    fam Add((P, _1  ), (Q, _1  )) => (Rec           , _0) {
        Add((p, _1{}), (q, _1{})) => (AddCarry(p, q), _0)
    } let {
        Rec = AddCarry(P, Q),
    } for .[ Rec ] :[ P: Pos, Q: Pos ]
}

// Infix: Add //////////////////////////////////////////////////////////////////

// unwrap/rewrap
impl<M: IsNat, N: IsNat, Rec: IsNat> std::ops::Add<W<N>> for W<M> where
    Add: Fn(M, N) -> Rec
{
    type Output = W<Rec>;
    #[inline]
    fn add(self, rhs: W<N>) -> W<Rec> {
        W(Add(self.0, rhs.0))
    }
}
