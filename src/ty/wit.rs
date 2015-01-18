use ty::fun;

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
pub fn wit<Fx: fun::Def, I: fun::Fn<Fx>>() -> Wit<fun::Ap<Fx, I>> { Wit(()) }
