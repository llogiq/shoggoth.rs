/// Heterogeneous lists
#[rustc_on_unimplemented = "`{Self}` is not a heterogeneous list"]
pub trait
    HList
{}

/// Empty heterogeneous list
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
    Nil;

impl
    HList
for
    Nil
{}

/// Cons heterogeneous list
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
    Cons<H, T>
where
    T: HList
{
    pub head: H,
    pub tail: T,
}

impl<H, T>
    HList
for
    Cons<H, T>
where
    T: HList
{}

/// `HList` predicate implemented when `Self` is heterogeneous cons
#[rustc_on_unimplemented = "`{Self}` is not a heterogeneous cons"]
pub trait
    IsCons
where
    Self: HList,
{
    type H;
    type T: HList;

    #[inline]
    fn head(self) -> Self::H;

    #[inline]
    fn tail(self) -> Self::T;
}

impl<
    H,
    T,
>
    IsCons
for
    Cons<H, T>
where
    T: HList,
{
    type H = H;
    type T = T;

    #[inline]
    fn head(self) -> H {
        self.head
    }

    #[inline]
    fn tail(self) -> T {
        self.tail
    }
}



pub trait
    Append<R>
where
    R: HList,
    Self: HList,
{
    type Out: HList;
}

impl<
    R,
>
    Append<R>
for
    Nil
where
    R: HList,
{
    type Out = R;
}

impl<
    H,
    R,
    T,
>
    Append<R>
for
    Cons<H, T>
where
    R: HList,
    T: Append<R>,
    T: HList,
{
    type Out = Cons<H, <T as Append<R>>::Out>;
}



pub trait
    Snoc<H>
where
    Self: HList,
{
    type Out: HList;
}

impl<
    X,
>
    Snoc<X>
for
    Nil
{
    type Out = Cons<X, Nil>;
}

impl<
    H,
    T,
    X,
>
    Snoc<X>
for
    Cons<H, T>
where
    T: HList,
    T: Snoc<X>,
{
    type Out = Cons<H, <T as Snoc<X>>::Out>;
}



trait
    ReverseHelper<Acc>
where
    Acc: HList,
    Self: HList,
{
    type Out: HList;
}

impl<
    Acc,
>
    ReverseHelper<Acc>
for
    Nil
where
    Acc: HList,
{
    type Out = Acc;
}

impl<
    Acc,
    H,
    T,
>
    ReverseHelper<Acc>
for
    Cons<H, T>
where
    Acc: HList,
    T: HList,
    T: ReverseHelper<Cons<H, Acc>>,
{
    type Out = <T as ReverseHelper<Cons<H, Acc>>>::Out;
}

pub trait
    Reverse
where
    Self: HList,
{
    type Out: HList;
}

impl<
    Xs
>
    Reverse
for
    Xs
where
    Xs: HList,
    Xs: ReverseHelper<Nil>,
{
    type Out = <Xs as ReverseHelper<Nil>>::Out;
}



/// Convenience alias for heterogeneous nil
pub type HN = Nil;

/// Convenience alias for heterogeneous cons
pub type HC<H, T> = Cons<H, T>;

/// Convenience alias for heterogeneous snoc
pub type HS<T, H> = <T as Snoc<H>>::Out;
