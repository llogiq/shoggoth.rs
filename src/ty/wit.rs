use ty::{
    Ap,
    FnTm,
    Sig,
};

/// A structure for witnessing a type-level computation
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Rand)]
#[derive(Show)]
pub struct Wit<A>(());

/// Compute a type-level expression by applying a "type-level
/// function" `F` to a type-level argument `I`
#[inline]
pub fn wit<S: Sig, I: FnTm<S>>() -> Wit<Ap<S, I>> { Wit(()) }
