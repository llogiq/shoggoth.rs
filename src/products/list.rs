#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Nil;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Cons<H, T: List>(pub H, pub T);

pub fn nil() -> Nil {
    Nil
}

#[rustc_on_unimplemented = "`{Self}` is not a heterogeneous list"]
pub trait List {
    fn cons<X>(self, x: X) -> Cons<X, Self> where Self: Sized {
        Cons(x, self)
    }
}
impl List for Nil {
}
impl<H, T: List> List for Cons<H, T> {
}

impl<Ys: List> ::std::ops::Add<Ys> for Nil {
    type Output = Ys;
    fn add(self, rhs: Ys) -> Ys {
        rhs
    }
}
impl<
    Rec: List + Sized,
    X,
    Xs: List,
    Ys: List,
> ::std::ops::Add<Ys> for Cons<X, Xs> where
    Xs: ::std::ops::Add<Ys, Output = Rec>,
{
    type Output = Cons<X, Rec>;
    fn add(self, rhs: Ys) -> Cons<X, Rec> {
        Cons(self.0, self.1 + rhs)
    }
}

pub type Append<Xs, Ys> = <Xs as ::std::ops::Add<Ys>>::Output;

pub type Single<X> = Cons<X, Nil>;

pub trait ToSingleton {
    type Out: List = Cons<Self, Nil>;
    fn single(self) -> Cons<Self, Nil> where
        Self: Sized,
    {
        Cons(self, Nil)
    }
}
impl<A> ToSingleton for A {
    type Out = Single<A>;
}
