use ty::{
    Tm,
    Ty,
};

/// Predicate classifying type-level "type signatures"
pub trait Sig { type Dom: Ty; type Cod: Ty; }

/// Predicate classifying type-level "relations as functions"
pub trait FnTm<S: Sig>: Tm<<S as Sig>::Dom> { type O: Tm<<S as Sig>::Cod>; }

/// Convenience type for type-level "function application"
// FIXME: pub type Ap<S: Sig, I: FnTm<S>> = <I as FnTm<S>>::O;
pub type Ap<S, I> = <I as FnTm<S>>::O;

/// Predicate classifying type-level "dependent functions": the output
/// type depends on the input type, driven by a type-level "function"
pub trait FnDep<S: Sig>: FnTm<S> {
    #[inline(always)]
    fn call(self) -> Ap<S, Self>;
}
