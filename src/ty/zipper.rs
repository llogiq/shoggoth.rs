use hlist::*;
use ty::{
    Tm,
    Ty,
};
use ty::fun::*;
use ty::list::*;



/// Type-level zippers for lists
pub enum Zipper<A> where
    A: Ty,
{}

/// ```ignore
/// A :: Ty
/// ---------------
/// Zipper[A] :: Ty
/// ```
impl<A> Ty for Zipper<A> where
    A: Ty,
{}



/// Zipper constructor
pub struct ZC<L, R>(L, R);

/// ```ignore
/// A :: Ty
/// l : List[A]
/// r : List[A]
/// -----------------------
/// zcons(l, r) : Zipper[A]
/// ```
impl<A, L, R> Tm<Zipper<A>> for ZC<L, R> where
    A: Ty,
    L: Tm<List<A>>,
    R: Tm<List<A>>,
{}



/// Type-level list to zipper
pub enum Zip<A: Ty> {}

/// ```ignore
/// A :: Ty
/// xs : List[A]
/// -------------------
/// zip(xs) : Zipper[A]
/// ```
impl<A: Ty> Sig for Zip<A> {
    type Dom = List<A>;
    type Cod = Zipper<A>;
}

/// `zip(xs) => zcons(nil, xs)`
impl<A, XS> Rule<Zip<A>> for XS where
    A: Ty,
    XS: Tm<List<A>>,
{
    type O = ZC<HN, XS>;
}



/// Type-level zipper to list
pub enum Unzip<A> where
    A: Ty,
{}

/// ```ignore
/// A :: Ty
/// zs : Zipper[A]
/// -------------------
/// unzip(zs) : List[A]
/// ```
impl<A> Sig for Unzip<A> where
    A: Ty,
{
    type Dom = Zipper<A>;
    type Cod = List<A>;
}

/// `unzip(zcons(l, r)) => append(l, r)`
impl<A, L, YS, Rec> Rule<Unzip<A>> for ZC<L, YS> where
    A: Ty,
    L: Tm<List<A>>,
    YS: Tm<List<A>>,
    Rec: Tm<List<A>>,
    HC<L, HC<YS, HN>>: Rule<Append<A>, O = Rec>,
{
    type O = Rec;
}



/// Type-level move right for zipper
pub enum Right<A> where
    A: Ty,
{}

/// ```ignore
/// A :: Ty
/// zs : Zipper[A]
/// ---------------------
/// right(zs) : Zipper[A]
/// ```
impl<A> Sig for Right<A> where
    A: Ty,
{
    type Dom = Zipper<A>;
    type Cod = Zipper<A>;
}

/// `right(zcons(l, cons(rh, rt))) => zcons(cons(rh, l), rt)`
impl<A, L, RH, RT> Rule<Right<A>> for ZC<L, HC<RH, RT>> where
    A: Ty,
    L: Tm<List<A>>,
    RH: Tm<A>,
    RT: Tm<List<A>> + HList,
{
    type O = ZC<HC<RH, L>, RT>;
}



/// Type-level move left for zipper
pub enum Left<A> where
    A: Ty,
{}

/// ```ignore
/// A :: Ty
/// zs : Zipper[A]
/// --------------------
/// left(zs) : Zipper[A]
/// ```
impl<A> Sig for Left<A> where
    A: Ty,
{
    type Dom = Zipper<A>;
    type Cod = Zipper<A>;
}

/// `left(zcons(cons(lh, lt), r)) => zcons(lt, cons(lh, r))`
impl<A, LH, LT, R> Rule<Left<A>> for ZC<HC<LH, LT>, R> where
    A: Ty,
    LH: Tm<A>,
    LT: Tm<List<A>>,
    R: Tm<List<A>> + HList,
{
    type O = ZC<LT, HC<LH, R>>;
}



/// Type-level get from focus of zipper
pub enum Get<A> where
    A: Ty,
{}

/// ```ignore
/// A :: Ty
/// zs : Zipper[A]
/// -----------
/// get(zs) : A
/// ```
impl<A> Sig for Get<A> where
    A: Ty,
{
    type Dom = Zipper<A>;
    type Cod = A;
}

/// `get(zcons(l, cons(rh, rt))) => rh`
impl<A, L, RH, RT> Rule<Get<A>> for ZC<L, HC<RH, RT>> where
    A: Ty,
    L: Tm<List<A>>,
    RH: Tm<A>,
    RT: Tm<List<A>> + HList,
{
    type O = RH;
}



/// Type-level put new element, replacing focus of zipper
pub enum Put<A> where
    A: Ty,
{}

/// ```ignore
/// A :: Ty
/// zs : Zipper[A]
/// e : A
/// ----------------------
/// put(zs, e) : Zipper[A]
/// ```
impl<A> Sig for Put<A> where
    A: Ty,
{
    type Dom = HC<Zipper<A>, HC<A, HN>>;
    type Cod = Zipper<A>;
}

/// `put(zcons(l, cons(rh, rt)), e) => zcons(l, cons(e, rt))`
impl<A, L, RH, RT, E> Rule<Put<A>> for HC<ZC<L, HC<RH, RT>>, HC<E, HN>> where
    A: Ty,
    L: Tm<List<A>>,
    RH: Tm<A>,
    RT: Tm<List<A>> + HList,
    E: Tm<A>,
{
    type O = ZC<L, HC<E, RT>>;
}



#[cfg(test)]
mod test {
    use hlist::*;
    use ty::*;

    #[test]
    fn put() {
        let x: Wit<HC<ZC<HC<Lift<bool>, HN>, HC<Lift<u8>, HN>>, HC<Lift<u16>, HN>>> = Wit;
        let x: Wit<ZC<HC<Lift<bool>, HN>, HC<Lift<u16>, HN>>> = x.app::<Put<Star>>();
        let _: Wit<HC<Lift<bool>, HC<Lift<u16>, HN>>> = x.app::<Unzip<Star>>();
    }
}
