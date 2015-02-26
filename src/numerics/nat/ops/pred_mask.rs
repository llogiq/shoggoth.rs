use numerics::bit::*;
use numerics::nat::*;
use numerics::nat::ops::*;

// Fn: PredMask ////////////////////////////////////////////////////////////////

/// `pred_mask(is_pos(1)) ==> is_nul`
ty! { #[inline]
    fam PredMask(mask::IsPos<_1  >,) => mask::IsNul {
        PredMask(mask::IsPos(_1{}),) => mask::IsNul
    }
}
/// `pred_mask(is_pos(p:b)) ==> is_pos(pred(p:b))`
ty! { #[inline]
    fam PredMask((P, B),) => (mask::IsPos<Rec>) {
        PredMask((p, b),) => (mask::IsPos(Pred((p, b))))
    } let {
        Rec = Pred((P, B),),
    } for :[ B: Bit, P: Pos, Rec: Pos ]
}
/// `pred_mask(is_nul) ==> is_neg`
ty! { #[inline]
    fam PredMask(mask::IsNul,) => mask::IsNeg {
        PredMask(mask::IsNul,) => mask::IsNeg
    }
}
/// `pred_mask(is_neg) ==> is_neg`
ty! { #[inline]
    fam PredMask(mask::IsNeg,) => mask::IsNeg {
        PredMask(mask::IsNeg,) => mask::IsNeg
    }
}
