pub mod bool;
pub mod fun;
pub mod list;
pub mod literal;
pub mod nat;
pub mod val;

/// Predicate classifying type-level "types"
pub trait Ty {}

/// Predicate classifying "typed" type-level "terms"
pub trait Tm<A: Ty> {}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Star {}

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

// `()` is a type
impl Ty for () {}

// Tuples comprised of types are types
impl<HTy: Ty, TTy: Ty, CTy> Ty for CTy where
    CTy: ::IsComposite<H = HTy, T = TTy>,
{}

// `Rust<A>` is a type-level term of type `Star`
impl<A> Tm<Star> for Rust<A> {}

// `()` is a type-level term of type `()`
impl Tm<()> for () {}

// Tuples comprised of type-level terms are terms
impl<HTy: Ty, HTm: Tm<HTy>, TTy: Ty, TTm: Tm<TTy>, CTy, CTm> Tm<CTy> for CTm where
    CTy: ::IsComposite<H = HTy, T = TTy>,
    CTm: ::IsComposite<H = HTm, T = TTm>,
{}
