use data::list;

// Data Definitions ////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Id<Xs>(pub Xs);

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Append<Xf, Yf>(pub Xf, pub Yf);

// Aliases /////////////////////////////////////////////////////////////////////

pub type Nil = Id<list::Nil>;
pub type Cons<X, Xf> = Append<Id<list::Single<X>>, Xf>;
pub type Snoc<Xf, X> = Append<Xf, Id<list::Single<X>>>;
pub type Single<X> = Id<list::Single<X>>;

pub fn nil() -> Nil {
    Id(list::Nil)
}

// Classifiers /////////////////////////////////////////////////////////////////

pub trait Diff {
    fn cons<X>(self, x: X) -> Cons<X, Self> where Self: Sized {
        Append::<Id<list::Cons<_, _>>, _>(Id(list::ToSingleton::single(x)), self)
    }

    fn snoc<X>(self, x: X) -> Snoc<Self, X> where Self: Sized {
        Append::<_, Id<list::Cons<_, _>>>(self, Id(list::ToSingleton::single(x)))
    }
}
impl<Xs> Diff for Id<Xs> {
    fn cons<X>(self, x: X) -> Cons<X, Self> where Self: Sized {
        Append::<Id<list::Cons<_, _>>, _>(Id(list::ToSingleton::single(x)), self)
    }
}
impl<Xf, Yf> Diff for Append<Xf, Yf> {
}

// Infix Operators /////////////////////////////////////////////////////////////

impl<Xs, Yf> ::std::ops::Add<Yf> for Id<Xs> {
    type Output = Append<Id<Xs>, Yf>;
    fn add(self, rhs: Yf) -> Append<Id<Xs>, Yf> {
        Append(self, rhs)
    }
}

impl<Xf, Yf, Zf> ::std::ops::Add<Zf> for Append<Xf, Yf> {
    type Output = Append<Append<Xf, Yf>, Zf>;
    fn add(self, rhs: Zf) -> Append<Append<Xf, Yf>, Zf> {
        Append(self, rhs)
    }
}

// Other Operators /////////////////////////////////////////////////////////////

pub trait ToDiff {
    fn diff(self) -> Id<Self> where Self: Sized {
        Id(self)
    }
}
impl ToDiff for list::Nil {
}
impl<X, Xs: ToDiff> ToDiff for list::Cons<X, Xs> {
}

pub type Eval<Xf, Ys> = <Xf as EvalOp<Ys>>::Output;

pub trait EvalOp<In>: Diff {
    type Output;
    fn eval(self, acc: In) -> Self::Output;
}

impl<Xs, Ys> EvalOp<Ys> for Id<Xs> where
    Xs: ::std::ops::Add<Ys>,
{
    type Output = list::Append<Xs, Ys>;
    fn eval(self, acc: Ys) -> list::Append<Xs, Ys> {
        self.0 + acc
    }
}

impl<Rec, Xf, Yf, Zs> EvalOp<Zs> for Append<Xf, Yf> where
    Xf: EvalOp<Rec>,
    Yf: EvalOp<Zs, Output = Rec>,
{
    type Output = Eval<Xf, Rec>;
    fn eval(self, acc: Zs) -> Eval<Xf, Rec> {
        (self.0).eval((self.1).eval(acc))
    }
}

pub trait ToList: EvalOp<list::Nil> {
    type Output;
    fn list(self) -> Eval<Self, list::Nil> where Self: Sized {
        self.eval(list::Nil)
    }
}

impl<Xf: EvalOp<list::Nil>> ToList for Xf {
    type Output = Eval<Xf, list::Nil>;
}

// Test ////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use data::list::{
        List,
        self,
    };
    use super::*;

    #[test]
    fn diff() {
        println!("{:?}", (list::Nil.cons(false).cons(42u64).cons("foo").diff()
                        + list::Nil.cons(false).cons(42u64).cons(vec![()]).diff()
                        + list::Nil.cons(false).cons(42u64).cons(()).diff().snoc("bar")
                        + list::Nil.cons("baz").cons(vec![0u8, 1u8]).cons(true).diff()).list());
    }
}
