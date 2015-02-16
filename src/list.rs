#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Nil;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Cons<H, T: List>(pub H, pub T);

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
impl List for Nil {}
impl<H, T: List> List for Cons<H, T> {}
