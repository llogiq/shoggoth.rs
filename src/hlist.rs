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
    HNil;

impl
    HList
for
    HNil
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
    HCons<H, T>
where
    T: HList
{
    pub head: H,
    pub tail: T,
}

impl<H, T>
    HList
for
    HCons<H, T>
where
    T: HList
{}

/// `HList` predicate implemented when `Self` is heterogeneous cons
#[rustc_on_unimplemented = "`{Self}` is not a heterogeneous cons"]
pub trait
    IsHCons
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
    IsHCons
for
    HCons<H, T>
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
