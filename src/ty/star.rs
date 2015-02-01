use hlist::*;
use ty::{
    Tm,
    Ty,
};

/// Type-level type of normal Rust types
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Ord)]
#[derive(Hash)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Star
{}

/// Normal Rust types lifted to terms at the type-level
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Lift<A>
{}

/// Lower Rust types lifted to type-level terms; inverse of `Lift`
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
     HTm: Tm<Star>,
     TTm: Tm<Star> + HList,
>
    Tm<Star>
for
    Cons<HTm, TTm>
{}
