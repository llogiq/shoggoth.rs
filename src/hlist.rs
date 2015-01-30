/// Heterogeneous lists
#[rustc_on_unimplemented = "`{Self}` is not a heterogeneous list"]
pub trait
    HList
{}

/// Empty heterogeneous list
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
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

/// Convenience alias for heterogeneous nil
pub type HN = Nil;

/// Convenience alias for heterogeneous cons
pub type HC<H, T> = Cons<H, T>;

/// Convenience alias for heterogeneous snoc
pub type HS<T, H> = <T as Snoc<H>>::Out;
