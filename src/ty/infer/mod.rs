use hlist::*;
use ty::{
    Arrow,
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
    C,
    D,
    M,
>
    Tm<Arrow<D, C>>
for
    M
where
    C: Ty,
    D: HList + Ty,
    M: Infer<Ty = Arrow<D, C>>,
{}
