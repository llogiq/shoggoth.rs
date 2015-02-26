use numerics::bit::*;
use numerics::nat::*;
use numerics::nat::ops::*;

// Fn: SubMaskCarry ////////////////////////////////////////////////////////////

/// `sub_mask_carry(1, q) ==> is_neg`
ty! { #[inline]
    fam SubMaskCarry(_1  , Q) => mask::IsNeg {
        SubMaskCarry(_1{}, _) => mask::IsNeg
    } for :[ Q: Pos ]
}
/// `sub_mask_carry((p:0), 1) ==> double_pred_mask(p)`
ty! { #[inline]
    fam SubMaskCarry((P, _0  ), _1  ) => Rec {
        SubMaskCarry((p, _0{}), _1{}) => DoublePredMask(p)
    } let {
        Rec = DoublePredMask(P,),
    } for :[ P: Pos, Rec: Pos ]
}
/// `sub_mask_carry((p:0), (q:0)) ==> succ_double_mask(sub_mask_carry p q)`
ty! { #[inline]
    fam SubMaskCarry((P, _0  ), (Q, _0  )) => Rec1 {
        SubMaskCarry((p, _0{}), (q, _0{})) => SuccDoubleMask(SubMaskCarry(p, q))
    } let {
        Rec0 = SubMaskCarry(P, Q),
        Rec1 = SuccDoubleMask(Rec0,),
    } for .[ Rec0, Rec1 ] :[ P: Pos, Q: Pos ]
}
/// `sub_mask_carry((p:0), (q:1)) ==> double_mask(sub_mask_carry(p, q))`
ty! { #[inline]
    fam SubMaskCarry((P, _0  ), (Q, _1  )) => Rec1 {
        SubMaskCarry((p, _0{}), (q, _1{})) => DoubleMask(SubMaskCarry(p, q))
    } let {
        Rec0 = SubMaskCarry(P, Q),
        Rec1 = DoubleMask(Rec0,),
    } for .[ Rec0, Rec1 ] :[ P: Pos, Q: Pos ]
}
/// `sub_mask((p:1), 1) ==> is_pos(pred_double(p))`
ty! { #[inline]
    fam SubMaskCarry((P, _1  ), _1  ) => mask::IsPos<Rec> {
        SubMaskCarry((p, _1{}), _1{}) => mask::IsPos(PredDouble(p))
    } let {
        Rec = PredDouble(P,),
    } for :[ P: Pos, Rec: Pos ]
}
/// `sub_mask_carry((p:1), (q:0)) ==> double_mask(sub_mask(p, q))`
ty! { #[inline]
    fam SubMaskCarry((P, _1  ), (Q, _0  )) => Rec1 {
        SubMaskCarry((p, _1{}), (q, _0{})) => DoubleMask(SubMask(p, q))
    } let {
        Rec0 = SubMask(P, Q),
        Rec1 = DoubleMask(Rec0,),
    } for .[ Rec0, Rec1 ] :[ P: Pos, Q: Pos ]
}
/// `sub_mask_carry((p:1), (q:1)) ==> double_mask(sub_mask(p, q))`
ty! { #[inline]
    fam SubMaskCarry((P, _1  ), (Q, _1  )) => Rec1 {
        SubMaskCarry((p, _1{}), (q, _1{})) => SuccDoubleMask(SubMaskCarry(p, q))
    } let {
        Rec0 = SubMaskCarry(P, Q),
        Rec1 = SuccDoubleMask(Rec0,),
    } for .[ Rec0, Rec1 ] :[ P: Pos, Q: Pos ]
}
