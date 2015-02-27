use bit::*;
use numerics::nat::bin::*;
use numerics::nat::bin::ops::*;
use order;

// Fn: Compare /////////////////////////////////////////////////////////////////

// unwrap
ty! { #[inline]
    fam Compare(W<M>, W<N>) => Rec {
        Compare(W(m), W(n)) => Compare(m, n)
    } let {
        Rec = Compare(M, N),
    } for .[ Rec ] :[ M: IsNat, N: IsNat ]
}

/// compare(0, 0) ==> eq
ty! { #[inline]
    fam Compare(_0  , _0  ) => order::Eq {
        Compare(_0{}, _0{}) => order::Eq
    }
}
/// compare(0, q) ==> lt
ty! { #[inline]
    fam Compare(_0  , Q) => order::LT {
        Compare(_0{}, _) => order::LT
    } for :[ Q: Pos ]
}
/// compare(p, 0) ==> gt
ty! { #[inline]
    fam Compare(P, _0  ) => order::GT {
        Compare(_, _0{}) => order::GT
    } for :[ P: Pos ]
}
/// compare(p, q) ==> compare_cont(p, q, eq)
ty! { #[inline]
    fam Compare(P, Q) => Rec {
        Compare(p, q) => CompareCont(p, q, order::Eq)
    } let {
        Rec = CompareCont(P, Q, order::Eq),
    } for .[ Rec ] :[ P: Pos, Q: Pos ]
}
