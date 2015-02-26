use bit::{
    Bit,
    _0,
    _1,
};
use nat::{
    Pos,
};
use nat::ops::*;

// Fn: SubMask /////////////////////////////////////////////////////////////////

/// `sub_mask(1, 1) ==> is_nul`
ty! { #[inline]
    fam SubMask(_1  , _1  ) => mask::IsNul {
        SubMask(_1{}, _1{}) => mask::IsNul
    }
}
/// `sub_mask(1, (q:b)) ==> is_neg`
ty! { #[inline]
    fam SubMask(_1  , (Q, B)) => mask::IsNeg {
        SubMask(_1{}, (_, _)) => mask::IsNeg
    } for :[ B: Bit, Q: Pos ]
}
/// `sub_mask((p:0), 1) ==> is_pos(pred_double(p))`
ty! { #[inline]
    fam SubMask((P, _0  ), _1  ) => mask::IsPos<Rec> {
        SubMask((p, _0{}), _1{}) => mask::IsPos(PredDouble(p))
    } let {
        Rec = PredDouble(P,),
    } for :[ P: Pos, Rec: Pos ]
}
/// `sub_mask((p:0), (q:0)) ==> double_mask(sub_mask(p, q))`
ty! { #[inline]
    fam SubMask((P, _0  ), (Q, _0  )) => Rec1 {
        SubMask((p, _0{}), (q, _0{})) => DoubleMask(SubMask(p, q))
    } let {
        Rec0 = SubMask(P, Q),
        Rec1 = DoubleMask(Rec0,),
    } for .[ Rec0, Rec1 ] :[ P: Pos, Q: Pos ]
}
/// `sub_mask((p:0), (q:1)) ==> succ_double_mask(sub_mask_carry(p, q))`
ty! { #[inline]
    fam SubMask((P, _0  ), (Q, _1  )) => Rec1 {
        SubMask((p, _0{}), (q, _1{})) => SuccDoubleMask(SubMaskCarry(p, q))
    } let {
        Rec0 = SubMaskCarry(P, Q),
        Rec1 = SuccDoubleMask(Rec0,),
    } for .[ Rec0, Rec1 ] :[ P: Pos, Q: Pos ]
}
/// `sub_mask((p:1), 1) ==> is_pos(p:0)`
ty! { #[inline]
    fam SubMask((P, _1  ), _1  ) => mask::IsPos<(P, _0)> {
        SubMask((p, _1{}), _1{}) => mask::IsPos((p, _0))
    } for :[ P: Pos ]
}
/// `sub_mask((p:1), (q:0)) ==> succ_double_mask(sub_mask(p, q))`
ty! { #[inline]
    fam SubMask((P, _1  ), (Q, _0  )) => Rec1 {
        SubMask((p, _1{}), (q, _0{})) => SuccDoubleMask(SubMask(p, q))
    } let {
        Rec0 = SubMask(P, Q),
        Rec1 = SuccDoubleMask(Rec0,),
    } for .[ Rec0, Rec1 ] :[ P: Pos, Q: Pos ]
}
/// `sub_mask((p:1), (q:1)) ==> double_mask(sub_mask(p, q))`
ty! { #[inline]
    fam SubMask((P, _1  ), (Q, _1  )) => Rec1 {
        SubMask((p, _1{}), (q, _1{})) => DoubleMask(SubMask(p, q))
    } let {
        Rec0 = SubMask(P, Q),
        Rec1 = DoubleMask(Rec0,),
    } for .[ Rec0, Rec1 ] :[ P: Pos, Q: Pos ]
}
