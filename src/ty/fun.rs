use ty::{
    Tm,
    Ty,
};
use unify;

/// Predicate classifying type-level "type signatures"
pub trait Sig { type Dom: Ty + ::IsComposite; type Cod: Ty; }

/// Predicate classifying type-level "function definitions"
pub trait Def: Sig {}
impl<Fx: Sig> Def for Fx {}

/// "Type-level functions"
pub trait Fn<Fx: Def>: Tm<<Fx as Sig>::Dom> { type O: Tm<<Fx as Sig>::Cod>; }

/// "Type-level function" application
pub type Ap<Fx: Def, I: self::Fn<Fx>> = <I as self::Fn<Fx>>::O;

// impl<Fx: Def, I: self::Fn<Fx>> Tm<<Fx as Sig>::Cod> for Ap<Fx, I> {}

/// "Dependent functions": the output type depends on the input type,
/// determined by a type-level function
pub trait DepFn<Fx: Def>: self::Fn<Fx> {
    #[inline(always)]
    fn call<X>(self) -> Ap<Fx, Self> where X: unify::Eq<Self>;
}
