use hlist::*;
use ty::{
    Ap,
    Ap1,
    Tm,
    Ty,
};

/// Type-level type of normal Rust types
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Star {}

/// Normal Rust types lifted to terms at the type-level
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Lift<A> {}
pub type L<A> = Lift<A>;

/// Lower Rust types lifted to type-level terms; inverse of `Lift`
#[rustc_on_unimplemented = "`{Self}` is not a lifted Rust type"]
pub trait Lower {
    type Out;
}

impl<A> Lower for Lift<A> {
    type Out = A;
}

/// ```ignore
/// ----------
/// Star :: Ty
/// ```
impl Ty for Star {}

/// ```ignore
/// A is a Rust type
/// --------------
/// Lift<A> : Star
/// ```
impl<A> Tm<Star> for Lift<A> {}

/// ```ignore
/// ----------
/// Nil : Star
/// ```
impl Tm<Star> for Nil {}

/// ```ignore
/// HTm : Star
/// TTm :: HList, TTm : Star
/// ---------------------
/// Cons<HTm, TTm> : Star
/// ```
impl<
     HTm: Tm<Star>,
     TTm: Tm<Star> + HList,
> Tm<Star> for Cons<HTm, TTm> {}

pub trait LiftMany: HList {
    type Out: Tm<Star> + HList;
}

impl LiftMany for Nil {
    type Out = Nil;
}

impl<X, Xs: HList> LiftMany for Cons<X, Xs> where Xs: LiftMany {
    type Out = Cons<Lift<X>, <Xs as LiftMany>::Out>;
}



pub trait LowerMany: Tm<Star> + HList {
    type Out: HList;
}

impl LowerMany for Nil {
    type Out = Nil;
}

impl<
       X: Tm<Star>,
      Xs: Tm<Star> + HList,
> LowerMany for Cons<X, Xs> where
       X: Lower,
      Xs: LowerMany,
{
    type Out = Cons<<X as Lower>::Out, <Xs as LowerMany>::Out>;
}

pub type ApT <Fx, Xs> = <Ap <Fx,  <Xs  as LiftMany>::Out> as Lower>::Out;
pub type ApT1<Fx, X > = <Ap1<Fx, L<X>>                    as Lower>::Out;
