pub use self::bit::{
    Bit,
    _0,
    _1,
};

pub use self::bool::{
    And,
    Bool,
    False,
    If,
    Not,
    Or,
    True,
};

pub use self::fun::{
    Ap,
    FnDep,
    FnTm,
    Sig,
};

pub use self::list::{
    Append,
    At,
    Cons,
    Length,
    List,
    Nil,
    ReplaceAt,
};

pub use self::wit::{
    Wit,
    wit,
};

use hlist::{
    HC,
    HList,
    HN,
};

mod bit;
mod bool;
mod fun;
mod list;
mod wit;

/// Type-level binary integers
pub mod int;

/// Type-level literals
pub mod literal;

/// Type-level natural numbers
pub mod nat;

/// Predicate classifying type-level "types"
#[rustc_on_unimplemented = "`{Self}` is not a valid type-level type"]
pub trait Ty {}

/// Predicate classifying "typed" type-level "terms"
#[rustc_on_unimplemented = "`{Self}` is not a valid type-level term"]
pub trait Tm<A: Ty> {}

/// `Star` classifies normal Rust types
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Star {}

/// `Rust<A>` makes a normal Rust type into a type-level term
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Rust<A> {}

// `Star` is a type
impl Ty for Star {}

// HNil is a type
impl Ty for HN {}

// HCons is a type
impl<HTy, TTy> Ty for HC<HTy, TTy> where
    HTy: Ty,
    TTy: Ty + HList,
{}

// `Rust<A>` is a type-level term of type `Star`
impl<A> Tm<Star> for Rust<A> {}

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
