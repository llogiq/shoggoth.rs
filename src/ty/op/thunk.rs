use hlist::*;
use ty::{
    Infer,
    Prefix,
    infer,
};
use ty::op::{
    Arrow,
    IsArrow,
};

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum
    Thunk<Fx, Xs>
where
    <Fx as Infer>::Ty: IsArrow,
    Fx: Infer,
    Xs: HList,
    Xs: Prefix<
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
    <Xs as Prefix<
        <<Fx as Infer>::Ty as IsArrow>::Dom>
    >::Out: HList,
    Fx: Infer,
    Xs: Prefix<
        <<Fx as Infer>::Ty as IsArrow>::Dom
    >,
{
    type Mode = infer::mode::Thunk;
    type Ty =
        Arrow<
            <Xs as Prefix<
                <<Fx as Infer>::Ty as IsArrow>::Dom>
            >::Out,
                <<Fx as Infer>::Ty as IsArrow>::Cod,
        >;
}
