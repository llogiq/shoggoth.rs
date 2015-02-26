use numerics::bit::*;
use numerics::nat::*;
use numerics::nat::ops::*;

// Fn: SuccDoubleMask //////////////////////////////////////////////////////////

// x -> 2 * x + 1

/// `succ_double_mask(is_nul) ==> is_pos(1)`
ty! { #[inline]
    fam SuccDoubleMask(mask::IsNul,) => mask::IsPos<_1> {
        SuccDoubleMask(mask::IsNul,) => mask::IsPos(_1)
    }
}
/// `succ_double_mask(is_neg) ==> is_neg`
ty! { #[inline]
    fam SuccDoubleMask(mask::IsNeg,) => mask::IsNeg {
        SuccDoubleMask(mask::IsNeg,) => mask::IsNeg
    }
}
/// `succ_double_mask(is_pos(p)) ==> is_pos(p:1)`
ty! { #[inline]
    fam SuccDoubleMask(mask::IsPos<P>,) => mask::IsPos<(P, _1)> {
        SuccDoubleMask(mask::IsPos(p),) => mask::IsPos((p, _1))
    } for :[ P: Pos ]
}
