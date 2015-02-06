use hlist::*;
use ty::{
    Infer,
    Tm,
    infer,
};
use ty::op::{
    IsArrow,
};

/// Interpret constants (i.e., operation symbols) at a given input
/// (i.e., `Self`)
#[rustc_on_unimplemented = "`{Fx}` cannot be evaluated at `{Self}`"]
pub trait Eval<Fx>
    : HList
    + Tm<<<Fx as Infer>::Ty as IsArrow>::Dom>
where
          Fx: Infer<Mode = infer::mode::Constant>,
    <Fx as Infer>::Ty
            : IsArrow,

{
    type Out: Tm<<<Fx as Infer>::Ty as IsArrow>::Cod>;
}

pub trait Eval1<Fx> where
              Fx: Infer,
    HC<Self, HN>: Eval<Fx>,
    HC<Self, HN>: Tm<<<Fx as Infer>::Ty as IsArrow>::Dom>,
           <Fx as Infer>::Ty
                : IsArrow,
{
    type Out: Tm<<<Fx as Infer>::Ty as IsArrow>::Cod>;
}

impl<Fx: Infer, Xs> Eval1<Fx> for Xs where
      HC<Xs, HN>: Eval<Fx>,
      HC<Xs, HN>: Tm<<<Fx as Infer>::Ty as IsArrow>::Dom>,
           <Fx as Infer>::Ty
                : IsArrow,
{
    type Out = <HC<Xs, HN> as Eval<Fx>>::Out;
}
