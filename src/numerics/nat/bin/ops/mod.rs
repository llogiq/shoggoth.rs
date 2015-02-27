pub mod mask {
    use numerics::nat::bin::*;
    use std::marker::*;

    #[doc(hidden)] pub struct IsNul;
    #[doc(hidden)] pub struct IsNeg;
    #[doc(hidden)] pub struct IsPos<P: Pos>(pub P);

    #[doc(hidden)]
    pub trait Mask: MarkerTrait {}
    impl Mask for IsNul {}
    impl Mask for IsNeg {}
    impl<P: Pos> Mask for IsPos<P> {}
}

#[doc(hidden)] pub struct Add;
#[doc(hidden)] pub struct AddCarry;
#[doc(hidden)] pub struct Compare;
#[doc(hidden)] pub struct CompareCont;
#[doc(hidden)] pub struct DoubleMask;
#[doc(hidden)] pub struct DoublePredMask;
#[doc(hidden)] pub struct Eq;
#[doc(hidden)] pub struct Pred;
#[doc(hidden)] pub struct PredDouble;
#[doc(hidden)] pub struct PredMask;
#[doc(hidden)] pub struct Sub;
#[doc(hidden)] pub struct SubCont;
#[doc(hidden)] pub struct SubMask;
#[doc(hidden)] pub struct SubMaskCarry;
#[doc(hidden)] pub struct Succ;
#[doc(hidden)] pub struct SuccDoubleMask;

pub mod add;
pub mod add_carry;
pub mod compare;
pub mod compare_cont;
pub mod double_mask;
pub mod double_pred_mask;
pub mod pred;
pub mod pred_double;
pub mod pred_mask;
pub mod succ;
pub mod succ_double_mask;
pub mod sub;
pub mod sub_cont;
pub mod sub_mask;
pub mod sub_mask_carry;
