pub use self::apply::{
    AppEval,
};
pub use self::eval::{
    Eval,
};
pub use self::thunk::{
    Thunk,
};
use hlist::*;
use ty::{
    Infer,
    Ty,
};

mod apply;
mod eval;
mod thunk;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum
    Arr<D, C>
where
    C: Ty,
    D: HList,
    D: Ty,
{}

impl<
    C,
    D,
>
    Ty
for
    Arr<D, C>
where
    C: Ty,
    D: HList,
    D: Ty,
{}

#[rustc_on_unimplemented = "`{Self}` is not a valid type-level arrow type"]
pub trait
    IsArr
where
    Self: Ty,
{
    type Dom: Ty + HList;
    type Cod: Ty;
}

impl<
    C,
    D,
>
    IsArr
for
    Arr<D, C>
where
    C: Ty,
    D: HList,
    D: Ty,
{
    type Dom = D;
    type Cod = C;
}

pub type Arr0   <C> = Arr<HN, C>;
pub type Arr1<D, C> = Arr<HC<D, HN>, C>;
pub type Ap<Fx, Xs> =
    <Xs as
        AppEval<
             <Fx as Infer>::Mode,
            <<Fx as Infer>::Ty as IsArr>::Dom,
            Fx,
        >
    >::Out;
pub type Ap1<Fx, X> = Ap<Fx, HC<X, HN>>;
