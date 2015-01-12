use unify;

/// "Type-level functions"
pub trait Fn<I> { type Out; }

/// "Type-level function" application
pub type Ap<F: self::Fn<I>, I> = <F as self::Fn<I>>::Out;

/// "Dependent functions": the output type depends on the input type,
/// determined by a type-level function
pub trait DepFn<I>: self::Fn<I> {
    #[inline(always)]
    fn call<X>(arg: I) -> Ap<Self, I> where X: unify::Eq<Self>;
}
