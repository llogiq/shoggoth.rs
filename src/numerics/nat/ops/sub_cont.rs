use bit::{
    _0,
};
use nat::{
    Pos,
};

// Fn: SubCont /////////////////////////////////////////////////////////////////

/// `sub_cont(is_neg) => 0`
ty! { #[inline]
    fam SubCont(mask::IsNeg,) => _0 {
        SubCont(mask::IsNeg,) => _0
    }
}
/// `sub_cont(is_nul) => 0`
ty! { #[inline]
    fam SubCont(mask::IsNul,) => _0 {
        SubCont(mask::IsNul,) => _0
    }
}
/// `sub_cont(is_pos(p)) => p`
ty! { #[inline]
    fam SubCont(mask::IsPos<P>,) => P {
        SubCont(mask::IsPos(p),) => p
    } for :[ P: Pos ]
}
