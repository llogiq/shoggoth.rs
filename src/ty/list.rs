use hlist::*;
use ty::{
    Rule,
    Sig,
    Tm,
    Ty,
};

/// Type-level lists
pub enum List<A> where
    A: Ty,
{}

/// ```ignore
/// A :: Ty
/// -------------
/// List[A] :: Ty
/// ```
impl<A> Ty for List<A> where
    A: Ty,
{}

/// ```ignore
/// A :: Ty
/// -------------
/// nil : List[A]
/// ```
impl<A> Tm<List<A>> for HNil where
    A: Ty,
{}

/// ```ignore
/// A :: Ty
/// h : A
/// t : List[A]
/// --------------------
/// cons(h, t) : List[A]
/// ```
impl<A, H, T> Tm<List<A>> for HCons<H, T> where
    A: Ty,
    H: Tm<A>,
    T: Tm<List<A>>,
{}



/// Type-level append for lists
pub enum Append<A: Ty> {}

/// ```ignore
/// A :: Ty
/// l : List[A]
/// r : List[A]
/// ----------------------
/// append(l, r) : List[A]
/// ```
impl<A: Ty> Sig for Append<A> {
    type Dom = HCons<List<A>, HCons<List<A>, HNil>>;
    type Cod = List<A>;
}

/// `append(nil, r) => r`
impl<A, R> Rule<Append<A>> for HCons<HNil, HCons<R, HNil>> where
    A: Ty,
    R: Tm<List<A>>,
{
    type Out = R;
}

/// `append(cons(lh, lt), r) => cons(lh, append(lt, r))`
impl<A, LH, LT, R, Rec> Rule<Append<A>> for HCons<HCons<LH, LT>, HCons<R, HNil>> where
    A: Ty,
    LH: Tm<A>,
    LT: Tm<List<A>> + HList,
    R: Tm<List<A>>,
    Rec: Tm<List<A>>,
    HCons<LT, HCons<R, HNil>>: Rule<Append<A>, Out = Rec>,
{
    type Out = HCons<LH, Rec>;
}
