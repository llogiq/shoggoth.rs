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
impl<A> Tm<List<A>> for HN where
    A: Ty,
{}

/// ```ignore
/// A :: Ty
/// h : A
/// t : List[A]
/// --------------------
/// cons(h, t) : List[A]
/// ```
impl<A, H, T> Tm<List<A>> for HC<H, T> where
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
    type Dom = HC<List<A>, HC<List<A>, HN>>;
    type Cod = List<A>;
}

/// `append(nil, r) => r`
impl<A, R> Rule<Append<A>> for HC<HN, HC<R, HN>> where
    A: Ty,
    R: Tm<List<A>>,
{
    type O = R;
}

/// `append(cons(lh, lt), r) => cons(lh, append(lt, r))`
impl<A, LH, LT, R, Rec> Rule<Append<A>> for HC<HC<LH, LT>, HC<R, HN>> where
    A: Ty,
    LH: Tm<A>,
    LT: Tm<List<A>> + HList,
    R: Tm<List<A>>,
    Rec: Tm<List<A>>,
    HC<LT, HC<R, HN>>: Rule<Append<A>, O = Rec>,
{
    type O = HC<LH, Rec>;
}
