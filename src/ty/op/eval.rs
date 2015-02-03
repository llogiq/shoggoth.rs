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
    <Fx as Infer
       >::Ty: IsArrow,
        Self: HList,
        Self: Tm<<<Fx as Infer>::Ty as IsArrow>::Dom>,
{
    type Out: Tm<<<Fx as Infer>::Ty as IsArrow>::Cod>;
}



pub trait
    Eval1<Fx>
where
              Fx: Infer,
    HC<Self, HN>: Eval<Fx>,
    HC<Self, HN>: Tm<<<Fx as Infer>::Ty as IsArrow>::Dom>,
           <Fx as Infer
           >::Ty: IsArrow,
{
    type Out: Tm<<<Fx as Infer>::Ty as IsArrow>::Cod>;
}

impl<
      Fx: Infer,
      Xs,
>
    Eval1<Fx>
for
    Xs
where
      HC<Xs, HN>: Eval<Fx>,
      HC<Xs, HN>: Tm<<<Fx as Infer>::Ty as IsArrow>::Dom>,
           <Fx as Infer
           >::Ty: IsArrow,
{
    type Out = <HC<Xs, HN> as Eval<Fx>>::Out;
}
