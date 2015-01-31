use hlist::*;
use ty::{
    Infer,
    TmPrefix,
    infer,
};
use ty::op::{
    Ar,
    IsArrow,
};

/// Partially applied type-level operations
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
    <Fx as Infer>::Ty: IsArrow,
    Fx: Infer,
    Xs: HList,
    Xs: TmPrefix<
        <<Fx as Infer>::Ty as IsArrow>::Dom
    >,
{}

impl<
    Fx,
    Xs,
>
    Infer
for
    Thunk<Fx, Xs>
where
    <Fx as Infer>::Ty: IsArrow,
    <Xs as TmPrefix<
        <<Fx as Infer>::Ty as IsArrow>::Dom>
    >::Out: HList,
    Fx: Infer,
    Xs: TmPrefix<
        <<Fx as Infer>::Ty as IsArrow>::Dom
    >,
{
    type Mode = infer::mode::Thunk;
    type Ty =
        Ar<
            <Xs as TmPrefix<
                <<Fx as Infer>::Ty as IsArrow>::Dom>
            >::Out,
                <<Fx as Infer>::Ty as IsArrow>::Cod,
        >;
}
