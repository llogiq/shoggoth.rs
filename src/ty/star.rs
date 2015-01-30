use hlist::*;
use ty::{
    Tm,
    Ty,
};

/// Type-level type of normal Rust types
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Ord)]
#[derive(Hash)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum
    Star
{}

/// Normal Rust types lifted to terms at the type-level
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum
    Lift<A>
{}

/// Type-level operation to project
#[rustc_on_unimplemented = "`{Self}` is not a lifted Rust type"]
pub trait
    Lower
{
    type Out;
}

impl<
    A,
>
    Lower
for
    Lift<A>
{
    type Out = A;
}

/// ```ignore
/// ----------
/// Star :: Ty
/// ```
impl
    Ty
for
    Star
{}



/// ```ignore
/// A is a Rust type
/// --------------
/// Lift<A> : Star
/// ```
impl<
    A,
>
    Tm<Star>
for
    Lift<A>
{}

/// ```ignore
/// ----------
/// Nil : Star
/// ```
impl 
    Tm<Star>
for
    Nil
{}

/// ```ignore
/// HTm : Star
/// TTm :: HList, TTm : Star
/// ---------------------
/// Cons<HTm, TTm> : Star
/// ```
impl<
    HTm,
    TTm,
>
    Tm<Star>
for
    Cons<HTm, TTm>
where
    HTm: Tm<Star>,
    TTm: HList,
    TTm: Tm<Star>
{}
