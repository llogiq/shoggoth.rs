use hlist::*;
use ty::{
    Infer,
    Tm,
};
use ty::op::{
    IsArrow,
};

/// Interpret constants (i.e., operation symbols) at a given input
/// (i.e., `Self`)
pub trait
    Eval<Fx>
where
    Fx: Infer,
    <Fx as Infer>::Ty: IsArrow,
    Self: HList,
    Self: Tm<<<Fx as Infer>::Ty as IsArrow>::Dom>,
{
    type Out: Tm<<<Fx as Infer>::Ty as IsArrow>::Cod>;
}
