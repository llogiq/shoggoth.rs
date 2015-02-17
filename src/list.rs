use std;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Nil;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Cons<H, T: List>(pub H, pub T);

#[macro_export] macro_rules! List {
    {} => { $crate::list::Nil };
    { $head:ty } => { $crate::list::Cons<$head, $crate::list::Nil> };
    { $head:ty, $($tail:ty),* } => { $crate::list::Cons<$head, List!($($tail),*)> };
}

#[macro_export] macro_rules! list {
    {} => { $crate::list::Nil };
    { $head:expr } => { $crate::list::Cons($head, $crate::list::Nil) };
    { $head:expr, $($tail:expr),* } => { $crate::list::Cons($head, list!($($tail),*)) };
}

#[macro_export] macro_rules! list_match {
    {} => { $crate::list::Nil };
    { $head:ident } => { $crate::list::Cons($head, $crate::list::Nil) };
    { $head:ident, $($tail:ident),* } => { $crate::list::Cons($head, list_match!($($tail),*)) };
}

pub trait ToSingleton {
    type Out: List = Cons<Self, Nil>;

    #[inline]
    fn single(self) -> Cons<Self, Nil> where Self: Sized {
        Cons(self, Nil)
    }
}

impl<A> ToSingleton for A {
    type Out = Cons<Self, Nil>;
}

#[rustc_on_unimplemented = "`{Self}` is not a heterogeneous list"]
pub trait List {
    #[inline]
    fn nil() -> Nil {
        Nil
    }

    #[inline]
    fn cons<X>(self, x: X) -> Cons<X, Self> where Self: Sized {
        Cons(x, self)
    }
}

impl List for Nil {
}

impl<H, T: List> List for Cons<H, T> {
}

impl<Ys: List> std::ops::Add<Ys> for Nil {
    type Output = Ys;

    #[inline]
    fn add(self, rhs: Ys) -> Ys {
        rhs
    }
}

impl<
    Rec: List + Sized,
    X,
    Xs: List,
    Ys: List,
> std::ops::Add<Ys> for Cons<X, Xs> where
    Xs: std::ops::Add<Ys, Output = Rec>,
{
    type Output = Cons<X, Rec>;

    #[inline]
    fn add(self, rhs: Ys) -> Cons<X, Rec> {
        Cons(self.0, self.1 + rhs)
    }
}

pub type Append<Xs, Ys> = <Xs as std::ops::Add<Ys>>::Output;
