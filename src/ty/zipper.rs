use hlist::*;
use ty::{
    Ar,
    Ar1,
    Eval,
    Infer,
    Tm,
    Ty,
    infer,
};
use ty::list::*;



/// Type-level zippers for lists
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
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
    A,
>
    Ty
for
    Zipper<A>
where
    A: Ty,
{}



/// Zipper constructor
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Rand)]
#[derive(Show)]
pub struct
    ZCons<LS, RS>(LS, RS)
where
    LS: HList,
    RS: HList;

/// ```ignore
/// A :: Ty
/// l : List<A>
/// r : List<A>
/// -----------------------
/// zcons(l, r) : Zipper<A>
/// ```
impl<
    A,
    L,
    R,
>
    Tm<Zipper<A>>
for
    ZCons<L, R>
where
    A: Ty,
    L: HList,
    L: Tm<List<A>>,
    R: HList,
    R: Tm<List<A>>,
{}



/// Type-level list to zipper
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
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
    A,
>
    Infer
for
    Zip<A>
where
    A: Ty,
{
    type Mode = infer::mode::Constant;
    type Ty   = Ar1<List<A>, Zipper<A>>;
}

/// `zip(xs) ==> zcons(nil, xs)`
impl<
    A,
    XS,
>
    Eval<Zip<A>>
for
    HC<XS, HN>
where
    A: Ty,
    XS: HList,
    XS: Tm<List<A>>,
{
    type Out = ZCons<HN, XS>;
}



/// Type-level zipper to list
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
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
    A,
>
    Infer
for
    Unzip<A>
where
    A: Ty,
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<Zipper<A>, List<A>>;
}

/// `unzip(zcons(l, r)) ==> append(l, r)`
impl<
    A,
    L,
    Rec,
    YS,
>
    Eval<Unzip<A>>
for
    HC<ZCons<L, YS>, HN>
where
    A: Ty,
    HC<L, HC<YS, HN>>: Eval<Append<A>, Out = Rec>,
    L: HList,
    L: Tm<List<A>>,
    Rec: Tm<List<A>>,
    YS: HList,
    YS: Tm<List<A>>,
{
    type Out = Rec;
}



/// Type-level move right for zipper
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
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
    A,
>
    Infer
for
    Right<A>
where
    A: Ty,
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<Zipper<A>, Zipper<A>>;
}

/// `right(zcons(l, cons(rh, rt))) ==> zcons(cons(rh, l), rt)`
impl<
    A,
    L,
    RH,
    RT,
>
    Eval<Right<A>>
for
    HC<ZCons<L, HC<RH, RT>>, HN>
where
    A: Ty,
    L: HList,
    L: Tm<List<A>>,
    RH: Tm<A>,
    RT: HList,
    RT: Tm<List<A>>,
{
    type Out = ZCons<HC<RH, L>, RT>;
}



/// Type-level move left for zipper
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
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
    A,
>
    Infer
for
    Left<A>
where
    A: Ty,
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<Zipper<A>, Zipper<A>>;
}

/// `left(zcons(cons(lh, lt), r)) ==> zcons(lt, cons(lh, r))`
impl<
    A,
    LH,
    LT,
    R,
>
    Eval<Left<A>>
for
    HC<ZCons<HC<LH, LT>, R>, HN>
where
    A: Ty,
    LH: Tm<A>,
    LT: HList,
    LT: Tm<List<A>>,
    R: HList,
    R: Tm<List<A>>,
{
    type Out = ZCons<LT, HC<LH, R>>;
}



/// Type-level get from focus of zipper
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
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
    A,
>
    Infer
for
    Get<A>
where
    A: Ty,
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<Zipper<A>, A>;
}

/// `get(zcons(l, cons(rh, rt))) ==> rh`
impl<
    A,
    L,
    RH,
    RT,
>
    Eval<Get<A>>
for
    HC<ZCons<L, HC<RH, RT>>, HN>
where
    A: Ty,
    L: HList,
    L: Tm<List<A>>,
    RH: Tm<A>,
    RT: HList,
    RT: Tm<List<A>>,
{
    type Out = RH;
}



/// Type-level put new element, replacing focus of zipper
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
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
    A,
>
    Infer
for
    Put<A>
where
    A: Ty,
{
    type Mode = infer::mode::Constant;
    type Ty = Ar<HC<Zipper<A>, HC<A, HN>>, Zipper<A>>;
}

/// `put(zcons(l, cons(rh, rt)), e) ==> zcons(l, cons(e, rt))`
impl<
    A,
    E,
    L,
    RH,
    RT,
>
    Eval<Put<A>>
for
    HC<ZCons<L, HC<RH, RT>>, HC<E, HN>>
where
    A: Ty,
    E: Tm<A>,
    L: HList,
    L: Tm<List<A>>,
    RH: Tm<A>,
    RT: HList,
    RT: Tm<List<A>>,
{
    type Out = ZCons<L, HC<E, RT>>;
}



#[cfg(test)]
mod test {
    use hlist::*;
    use ty::*;

    #[test]
    fn put() {
        let x0: Witness<
            Ap<
                Put<Star>,
                HC<
                    ZCons<
                        HC<Lift<bool>, HN>,
                        HC<Lift<u8>  , HN>
                    >,
                    HC<Lift<u16>, HN>
                >
            >
        > = Witness;
        let x1: Witness<
            ZCons<
                HC<Lift<bool>, HN>,
                HC<Lift<u16> , HN>
            >
        > = Witness;
        x0 == x1;
        let x2: Witness<
            Ap1<
                Unzip<Star>,
                ZCons<
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
