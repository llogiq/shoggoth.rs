/// Heterogeneous lists
#[rustc_on_unimplemented = "`{Self}` is not a heterogeneous list"]
pub trait
    HList
{}

/// Empty heterogeneous list
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
    Nil;

impl
    HList
for
    Nil
{}

/// Cons heterogeneous list
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
       T: HList,
>
    IsCons
for
    Cons<H, T>
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



/// Append for heterogeneous lists
pub trait
    Append<R>
where
       R: HList,
    Self: HList,
{
    type Out: HList;
}

impl<
    R: HList,
>
    Append<R>
for
    Nil
{
    type Out = R;
}

impl<
       H,
       R: HList,
       T: HList,
>
    Append<R>
for
    Cons<H, T>
where
       T: Append<R>,
{
    type Out = Cons<H, <T as Append<R>>::Out>;
}



/// Snoc (cons to tail) for heterogeneous lists
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
       T: HList,
       X,
>
    Snoc<X>
for
    Cons<H, T>
where
    T: Snoc<X>,
{
    type Out = Cons<H, <T as Snoc<X>>::Out>;
}



pub trait
    AppendReverse<Acc>
where
     Acc: HList,
    Self: HList,
{
    type Out: HList;
}

impl<
     Acc: HList,
>
    AppendReverse<Acc>
for
    Nil
{
    type Out = Acc;
}

impl<
     Acc: HList,
       H,
       T: HList,
>
    AppendReverse<Acc>
for
    Cons<H, T>
where
       T: AppendReverse<Cons<H, Acc>>,
{
    type Out = <T as AppendReverse<Cons<H, Acc>>>::Out;
}

/// Reverse for heterogeneous lists
pub trait
    Reverse
where
    Self: HList,
{
    type Out: HList;
}

impl<
      Xs: HList,
>
    Reverse
for
    Xs
where
      Xs: AppendReverse<Nil>,
{
    type Out = <Xs as AppendReverse<Nil>>::Out;
}



/// Alias for heterogeneous nil
pub type HN = Nil;

/// Alias for heterogeneous cons
pub type HC<H, T> = Cons<H, T>;

/// Alias for heterogeneous snoc
pub type HS<T, H> = <T as Snoc<H>>::Out;
