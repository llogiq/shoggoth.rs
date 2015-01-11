use unify;

/// "Type-level functions"
pub trait Fn<I> { type O; }

/// "Type-level function" application
pub type Ap<F: self::Fn<I>, I> = <F as self::Fn<I>>::O;

/// "Dependent functions": the output type depends on the input type,
/// determined by a type-level function
pub trait DepFn<I>: self::Fn<I> {
    #[inline(always)]
    fn call<X>(arg: I) -> Ap<Self, I> where X: unify::Eq<Self>;
}

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
pub struct Val<A>(());

/// Compute a type-level expression by applying a "type-level
/// function" `F` to a type-level argument `I`
#[inline]
pub fn val<F: self::Fn<I>, I>() -> Val<Ap<F, I>> { Val(()) }
