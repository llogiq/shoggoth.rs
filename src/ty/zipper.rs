use hlist::{
    HC,
    HList,
    HN,
    self,
};
use ty::{
    Ar,
    Ar1,
    Eval,
    Infer,
    List,
    Tm,
    Ty,
    infer,
};



/// Type-level zippers for lists
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Zipper<A>
where
       A: Ty,
{}

/// ```ignore
/// A :: Ty
/// ---------------
/// Zipper<A> :: Ty
/// ```
impl<
       A: Ty,
>
    Ty
for
    Zipper<A>
{}



/// Zipper constructor
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Rand)]
pub struct
    MkZipper<Xs, Ys>(Xs, Ys)
where
      Xs: HList,
      Ys: HList;

/// ```ignore
/// A :: Ty
/// xs : List<A>
/// ys : List<A>
/// -----------------------------
/// mk_zipper(xs, ys) : Zipper<A>
/// ```
impl<
       A: Ty,
      Xs: Tm<List<A>> + HList,
      Ys: Tm<List<A>> + HList,
>
    Tm<Zipper<A>>
for
    MkZipper<Xs, Ys>
{}



/// Type-level list to zipper
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Zip<A>
where
       A: Ty,
{}

/// ```ignore
/// A :: Ty
/// xs : List<A>
/// -------------------
/// zip(xs) : Zipper<A>
/// ```
impl<
       A: Ty,
>
    Infer
for
    Zip<A>
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<List<A>, Zipper<A>>;
}

/// `zip(xs) ==> mk_zipper(nil, xs)`
impl<
       A: Ty,
      XS: Tm<List<A>> + HList,
>
    Eval<Zip<A>>
for
    HC<XS, HN>
{
    type Out = MkZipper<HN, XS>;
}



/// Type-level zipper to list
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Unzip<A>
where
       A: Ty,
{}

/// ```ignore
/// A :: Ty
/// zs : Zipper<A>
/// -------------------
/// unzip(zs) : List<A>
/// ```
impl<
       A: Ty,
>
    Infer
for
    Unzip<A>
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<Zipper<A>, List<A>>;
}

/// `unzip(mk_zipper(xs, ys)) ==> append(reverse(xs), ys)`
impl<
       A: Ty,
      Xs: Tm<List<A>> + HList,
      Ys: Tm<List<A>> + HList,
     Rec: Tm<List<A>>,
>
    Eval<Unzip<A>>
for
    HC<MkZipper<Xs, Ys>, HN>
where
      Xs: hlist::AppendReverse<Ys, Out = Rec>,
{
    type Out = Rec;
}



/// Type-level move right for zipper
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Right<A>
where
       A: Ty,
{}

/// ```ignore
/// A :: Ty
/// zs : Zipper<A>
/// ---------------------
/// right(zs) : Zipper<A>
/// ```
impl<
       A: Ty,
>
    Infer
for
    Right<A>
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<Zipper<A>, Zipper<A>>;
}

/// `right(mk_zipper(xs, cons(y, ys))) ==> mk_zipper(cons(y, xs), ys)`
impl<
       A: Ty,
      Xs: Tm<List<A>> + HList,
       Y: Tm<A>,
      Ys: Tm<List<A>> + HList,
>
    Eval<Right<A>>
for
    HC<MkZipper<Xs, HC<Y, Ys>>, HN>
{
    type Out = MkZipper<HC<Y, Xs>, Ys>;
}



/// Type-level move left for zipper
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Left<A>
where
       A: Ty,
{}

/// ```ignore
/// A :: Ty
/// zs : Zipper<A>
/// --------------------
/// left(zs) : Zipper<A>
/// ```
impl<
       A: Ty,
>
    Infer
for
    Left<A>
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<Zipper<A>, Zipper<A>>;
}

/// `left(mk_zipper(cons(x, xs), ys)) ==> mk_zipper(xs, cons(x, ys))`
impl<
       A: Ty,
       X: Tm<A>,
      Xs: Tm<List<A>> + HList,
      Ys: Tm<List<A>> + HList,
>
    Eval<Left<A>>
for
    HC<MkZipper<HC<X, Xs>, Ys>, HN>
{
    type Out = MkZipper<Xs, HC<X, Ys>>;
}



/// Type-level get from focus of zipper
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Get<A>
where
       A: Ty,
{}

/// ```ignore
/// A :: Ty
/// zs : Zipper<A>
/// -----------
/// get(zs) : A
/// ```
impl<
       A: Ty,
>
    Infer
for
    Get<A>
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<Zipper<A>, A>;
}

/// `get(mk_zipper(xs, cons(y, ys))) ==> y`
impl<
       A: Ty,
      Xs: Tm<List<A>> + HList,
       Y: Tm<A>,
      Ys: Tm<List<A>> + HList,
>
    Eval<Get<A>>
for
    HC<MkZipper<Xs, HC<Y, Ys>>, HN>
{
    type Out = Y;
}



/// Type-level put new element, replacing focus of zipper
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Put<A>
where
       A: Ty,
{}

/// ```ignore
/// A :: Ty
/// zs : Zipper<A>
/// e : A
/// ----------------------
/// put(zs, e) : Zipper<A>
/// ```
impl<
       A: Ty,
>
    Infer
for
    Put<A>
{
    type Mode = infer::mode::Constant;
    type Ty = Ar<HC<Zipper<A>, HC<A, HN>>, Zipper<A>>;
}

/// `put(mk_zipper(xs, cons(y, ys)), e) ==> mk_zipper(xs, cons(e, ys))`
impl<
       A: Ty,
       E: Tm<A>,
      Xs: Tm<List<A>> + HList,
       Y: Tm<A>,
      Ys: Tm<List<A>> + HList,
>
    Eval<Put<A>>
for
    HC<MkZipper<Xs, HC<Y, Ys>>, HC<E, HN>>
{
    type Out = MkZipper<Xs, HC<E, Ys>>;
}



#[cfg(test)]
mod test {
    use hlist::{
        HC,
        HN,
    };
    use ty::*;

    #[test]
    fn put() {
        let x0: Witness<
            Ap<
                Put<Star>,
                HC<
                    MkZipper<
                        HC<Lift<bool>, HN>,
                        HC<Lift<u8>  , HN>
                    >,
                    HC<Lift<u16>, HN>
                >
            >
        > = Witness;
        let x1: Witness<
            MkZipper<
                HC<Lift<bool>, HN>,
                HC<Lift<u16> , HN>
            >
        > = Witness;
        x0 == x1;
        let x2: Witness<
            Ap1<
                Unzip<Star>,
                MkZipper<
                    HC<Lift<bool>, HN>,
                    HC<Lift<u16> , HN>
                >
            >
        > = Witness;
        let x3: Witness<
            HC<Lift<bool>, HC<Lift<u16>, HN>>
        > = Witness;
        x2 == x3;
    }
}
