use hlist::*;
use ty::{
    Infer,
    Prefix,
    infer,
};
use ty::op::{
    Arr,
    IsArr,
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
    <Fx as Infer>::Ty: IsArr,
    Fx: Infer,
    Xs: HList,
    Xs: Prefix<
        <<Fx as Infer>::Ty as IsArr>::Dom
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
    <Fx as Infer>::Ty: IsArr,
    <Xs as Prefix<
        <<Fx as Infer>::Ty as IsArr>::Dom>
    >::Out: HList,
    Fx: Infer,
    Xs: Prefix<
        <<Fx as Infer>::Ty as IsArr>::Dom
    >,
{
    type Mode = infer::mode::Thunk;
    type Ty =
        Arr<
            <Xs as Prefix<
                <<Fx as Infer>::Ty as IsArr>::Dom>
            >::Out,
                <<Fx as Infer>::Ty as IsArr>::Cod,
        >;
}
