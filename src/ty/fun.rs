use hlist::{
    HC,
    HN,
};
use ty::{
    Lift,
    Lower,
    Star,
    Tm,
    Ty,
};

#[rustc_on_unimplemented = "`{Self}` is missing a type-level signature"]
pub trait Sig { type Dom: Ty; type Cod: Ty; }

#[rustc_on_unimplemented = "Type operation `{Op}` must be defined for input term `{Self}`"]
pub trait Rule<Op: Sig>: Tm<<Op as Sig>::Dom> { type O: Tm<<Op as Sig>::Cod>; }



pub enum Arr<A: Ty, B: Ty> {}

impl<A: Ty, B: Ty> Ty for Arr<A, B> {}

pub enum Fun<Op: Sig> {}
impl<Op: Sig> Sig for Fun<Op> { type Dom = <Op as Sig>::Dom; type Cod = <Op as Sig>::Cod; }
impl<A: Ty, B: Ty, Op> Tm<Arr<A, B>> for Fun<Op> where




    Op: Sig<Dom = A, Cod = B>,
{}

enum Split<A: Ty, B: Ty> {}
impl<A: Ty, B: Ty> Sig for Split<A, B> { type Dom = HC<Arr<A, B>, HC<A, HN>>; type Cod = B; }
impl<A, B, Op, Rec, X> Rule<Split<A, B>> for HC<Fun<Op>, HC<X, HN>> where


    A: Ty,
    B: Ty,
    Op: Sig<Dom = A, Cod = B>,
    Rec: Tm<B>,
    X: Rule<Op, O = Rec>,
{
    type O = Rec;
}

/// Convenience alias for type-level "function application"
pub type App<Fx, X> = <HC<Fx, HC<X, HN>> as Rule<Split<<Fx as Sig>::Dom, <Fx as Sig>::Cod>>>::O;

/// Convenience alias for type-level "function application" with


/// automatic lifting and lowering.
pub type AppL<Fx, X> = <App<Fx, Lift<X>> as Lower>::O;

pub trait Gen<Fx: Tm<Arr<Star, Star>>> where Lift<Self>: Tm<Star>
{
    #[inline(always)]
    fn call(self) -> AppL<Fx, Lift<Self>>;
}
