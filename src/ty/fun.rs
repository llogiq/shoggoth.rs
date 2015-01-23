use hlist::{
    HCons,
    HNil,
};
use ty::{
    Lift,
    Rust,
    Star,
    Tm,
    Ty,
};

/// Signature for a type-level partial operation from domain
/// `Self::Dom` to codomain `Self::Cod`
#[rustc_on_unimplemented = "`{Self}` is missing a type-level signature"]
pub trait Sig {
    type Dom: Ty;
    type Cod: Ty;
}

/// Individual rule for a type-level partial operation `Op` for the
/// input term given as `Self`
#[rustc_on_unimplemented = "Type operation `{Op}` must be defined for input term `{Self}`"]
pub trait Rule<Op: Sig>: Tm<<Op as Sig>::Dom> {
    type Out: Tm<<Op as Sig>::Cod>;
}



/// `Arr<A, B>` classifies type-level partial operation as partial
/// actions from type A to type B
pub enum Arr<A: Ty, B: Ty> {}

/// ```ignore
/// A :: Ty
/// B :: Ty
/// ---------------
/// A -> B :: Ty
/// ```
impl<A: Ty, B: Ty> Ty for Arr<A, B> {}



/// `Act<Op>` is a type-level term standing for a type-level partial
/// operation `Op`
pub enum Act<Op: Sig> {}

// `Act<Op>` has the same `Sig` as `Op`
impl<Op: Sig> Sig for Act<Op> { type Dom = <Op as Sig>::Dom; type Cod = <Op as Sig>::Cod; }

/// ```ignore
/// A :: Ty
/// B :: Ty
/// x : A |- op(x) : B
/// ----------------
/// act(op) : A -> B
/// ```
impl<A: Ty, B: Ty, Op> Tm<Arr<A, B>> for Act<Op> where
    Op: Sig<Dom = A, Cod = B>,
{}

/// Type-level partial operation for evaluating another type-level
/// partial operation referred to by an action term
enum EvalAt<A: Ty, B: Ty> {}

/// ```ignore
/// A :: Ty
/// B :: Ty
/// f : A -> B
/// x : A
/// ----------------------
/// evalAt[A, B](f, x) : B
/// ```
impl<A: Ty, B: Ty> Sig for EvalAt<A, B> { type Dom = HCons<Arr<A, B>, HCons<A, HNil>>; type Cod = B; }

/// `evalAt[A, B](f, x) = <eval f at x>`
impl<A, B, Op, Rec, X> Rule<EvalAt<A, B>> for HCons<Act<Op>, HCons<X, HNil>> where
    A: Ty,
    B: Ty,
    Op: Sig<Dom = A, Cod = B>,
    Rec: Tm<B>,
    X: Rule<Op, Out = Rec>,
{
    type Out = Rec;
}



/// Convenience alias for evaluating type-level partial operations
pub type Eval<Fx, X> = <HCons<Fx, HCons<X, HNil>> as Rule<EvalAt<<Fx as Sig>::Dom, <Fx as Sig>::Cod>>>::Out;

/// Convenience alias for type-level partial operations with
/// automatic lifting and lowering.
pub type Lower<Fx, X> = <Eval<Fx, Lift<X>> as Rust>::O;

pub trait Dep<Fx: Tm<Arr<Star, Star>>> where Lift<Self>: Tm<Star>
{
    #[inline(always)]
    fn call(self) -> Lower<Fx, Lift<Self>>;
}
