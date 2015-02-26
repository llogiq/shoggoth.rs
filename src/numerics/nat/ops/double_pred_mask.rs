use numerics::bit::*;
use numerics::nat::*;
use numerics::nat::ops::*;

// Fn: DoublePredMask //////////////////////////////////////////////////////////

// x -> 2 * x - 2

/// `double_pred_mask(1) ==> is_nul`
ty! { #[inline]
    fam DoublePredMask(mask::IsNul,) => mask::IsNul {
        DoublePredMask(mask::IsNul,) => mask::IsNul
    }
}
/// `double_pred_mask(p:0) ==> is_pos(pred_double(p)):0`
ty! { #[inline]
    fam DoublePredMask((P, _0  ),) => (mask::IsPos<Rec>          , _0) {
        DoublePredMask((p, _0{}),) => (mask::IsPos(PredDouble(p)), _0)
    } let {
        Rec = PredDouble(P,),
    } for :[ P: Pos, Rec: Pos ]
}
/// `double_pred_mask(p:1) ==> is_pos(p:0:0)`
ty! { #[inline]
    fam DoublePredMask((P, _1  ),) => mask::IsPos<((P, _0), _0)> {
        DoublePredMask((p, _1{}),) => mask::IsPos(((p, _0), _0))
    } for :[ P: Pos ]
}
