use hlist::*;
use ty::{
    Infer,
    TmPrefix,
    Ty,
    infer,
};
use ty::op::{
    Ar,
    IsArrow,
};

/// Curried type-level operations
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Thunk<Fx, Xs>
where
    <Fx as Infer>::Ty
        : IsArrow,
      Fx: Infer,
      Xs: HList,
      Xs: TmPrefix<<<Fx as Infer>::Ty as IsArrow>::Dom>,
{}

impl<
       C: Ty,
       D: Ty + HList,
      Ds: Ty + HList,
      Fx: Infer<Ty = Ar<D, C>>,
      Xs: HList,
>
    Infer
for
    Thunk<Fx, Xs>
where
      Xs: TmPrefix<D, Out = Ds>,
{
    type Mode = infer::mode::Thunk;
    type Ty = Ar<Ds, C>;
}
