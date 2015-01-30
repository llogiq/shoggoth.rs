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
    Arrow<D, C>
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
    Arrow<D, C>
where
    C: Ty,
    D: HList,
    D: Ty,
{}

#[rustc_on_unimplemented = "`{Self}` is not a valid type-level arrow type"]
pub trait
    IsArrow
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
    IsArrow
for
    Arrow<D, C>
where
    C: Ty,
    D: HList,
    D: Ty,
{
    type Dom = D;
    type Cod = C;
}

pub type Arrow0   <C> = Arrow<HN, C>;
pub type Arrow1<D, C> = Arrow<HC<D, HN>, C>;
pub type Ap<Fx, Xs> =
    <Xs as
        AppEval<
             <Fx as Infer>::Mode,
            <<Fx as Infer>::Ty as IsArrow>::Dom,
            Fx,
        >
    >::Out;
pub type Ap1<Fx, X> = Ap<Fx, HC<X, HN>>;
