pub use self::bit::{
    Bit,
    _0,
    _1,
};
pub use self::bool::{
    And,
    Bool,
    FF,
    If,
    Not,
    Or,
    TT,
};
pub use self::fun::{
    App,
    Arr,
    Fun,
    Gen,
    Rule,
    Sig,
};
pub use self::wit::{
    Wit,
    app,
};
use hlist::{
    HC,
    HList,
    HN,
};

mod bit;
mod bool;
mod fun;
mod wit;

/// Type-level binary integers
pub mod int;

/// Type-level natural numbers
pub mod nat;

/// Predicate classifying type-level "types"
#[rustc_on_unimplemented = "`{Self}` is not a valid type-level type"]
pub trait Ty {}

/// Predicate classifying "typed" type-level "terms"
#[rustc_on_unimplemented = "`{Self}` is not a valid type-level term"]
pub trait Tm<A: Ty> {}

/// `Star` classifies normal Rust types
pub enum Star {}

/// `Lift<A>` makes a normal Rust type into a type-level term
pub enum Lift<A> {}
trait Lower { type O; }
impl<A> Lower for Lift<A> { type O = A; }

// `Star` is a type
impl Ty for Star {}

// HNil is a type
impl Ty for HN {}

// HCons is a type
impl<HTy, TTy> Ty for HC<HTy, TTy> where
    HTy: Ty,
    TTy: Ty + HList,
{}

// `Lift<A>` is a type-level term of type `Star`
impl<A> Tm<Star> for Lift<A> {}

// HNil is a type-level term of type Star
impl Tm<Star> for HN {}

// HCons is a type-level term of type Star
impl<HTm: Tm<Star>, TTm: Tm<Star> + HList> Tm<Star> for HC<HTm, TTm> {}

// HNil is a term
impl Tm<HN> for HN {}

// HCons is a type-level term
impl<HTy, TTy, HTm, TTm> Tm<HC<HTy, TTy>> for HC<HTm, TTm> where
    HTy: Ty,
    TTy: Ty + HList,
    HTm: Tm<HTy>,
    TTm: Tm<TTy> + HList,
{}

pub type     _0b = bit::_0;
pub type     _1b = bit::_1;
pub type     _2b = (    _1b, _0b);
pub type     _4b = (    _2b, _0b);
pub type     _8b = (    _4b, _0b);
pub type    _16b = (    _8b, _0b);
pub type    _32b = (   _16b, _0b);
pub type    _64b = (   _32b, _0b);
pub type   _128b = (   _64b, _0b);
pub type   _256b = (  _128b, _0b);
pub type   _512b = (  _256b, _0b);
pub type  _1024b = (  _512b, _0b);
pub type  _2048b = ( _1024b, _0b);
pub type  _4096b = ( _2048b, _0b);
pub type  _8192b = ( _4096b, _0b);
pub type _16384b = ( _8192b, _0b);
pub type _32768b = (_16384b, _0b);
pub type _65536b = (_32768b, _0b);
