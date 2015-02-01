use hlist::*;
use ty::{
    Ar,
    Tm,
    Ty,
};

/// Infer modes for controlling `AppEval` behavior
pub mod mode;

/// Predicate classifying type-level type-inferrable terms
#[rustc_on_unimplemented = "`{Self}` is not a valid type-level type-inferrable term"]
pub trait
    Infer
{
    type Mode: mode::Mode;
    type Ty: Ty;
}

impl<
       C: Ty,
       D: Ty + HList,
       M: Infer<Ty = Ar<D, C>>,
>
    Tm<Ar<D, C>>
for
    M
{}
