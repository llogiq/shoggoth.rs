use list::{
    List,
    self,
};

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Id<Xs: List>(pub Xs);

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Append<Xf: Diff, Yf: Diff>(pub Xf, pub Yf);

pub type Nil = Id<list::Nil>;
pub type Cons<X, Xf> = Append<Id<list::Single<X>>, Xf>;
pub type Snoc<Xf, X> = Append<Xf, Id<list::Single<X>>>;
pub type Single<X> = Id<list::Single<X>>;

#[inline]
pub fn nil() -> Nil {
    Id(list::nil())
}

pub trait Diff {
    #[inline]
    fn cons<X>(self, x: X) -> Cons<X, Self> where
        Self: Sized,
    {
        Append::<Id<list::Cons<_, _>>, _>(Id(list::ToSingleton::single(x)), self)
    }

    #[inline]
    fn snoc<X>(self, x: X) -> Snoc<Self, X> where
        Self: Sized,
    {
        Append::<_, Id<list::Cons<_, _>>>(self, Id(list::ToSingleton::single(x)))
    }
}
impl<Xs: List> Diff for Id<Xs> {
    #[inline]
    fn cons<X>(self, x: X) -> Cons<X, Self> where
        Self: Sized,
    {
        Append::<Id<list::Cons<_, _>>, _>(Id(list::ToSingleton::single(x)), self)
    }
}
impl<Xf: Diff, Yf: Diff> Diff for Append<Xf, Yf> {
}

pub trait ToDiff: List {
    #[inline]
    fn diff(self) -> Id<Self> where
        Self: Sized,
    {
        Id(self)
    }
}
impl<Xs: List> ToDiff for Xs {
}

impl<Xs: List, Yf: Diff> ::std::ops::Add<Yf> for Id<Xs> {
    type Output = Append<Id<Xs>, Yf>;

    #[inline]
    fn add(self, rhs: Yf) -> Append<Id<Xs>, Yf> {
        Append(self, rhs)
    }
}

impl<Xf: Diff, Yf: Diff, Zf: Diff> ::std::ops::Add<Zf> for Append<Xf, Yf> {
    type Output = Append<Append<Xf, Yf>, Zf>;

    #[inline]
    fn add(self, rhs: Zf) -> Append<Append<Xf, Yf>, Zf> {
        Append(self, rhs)
    }
}

pub type Eval<Xf, Ys> = <Xf as EvalOp<Ys>>::Output;

pub trait EvalOp<In: List>: Diff {
    type Output;

    fn eval(self, acc: In) -> Self::Output;
}

impl<
    Xs: List,
    Ys: List
> EvalOp<Ys> for Id<Xs> where
    Xs: ::std::ops::Add<Ys>,
{
    type Output = list::Append<Xs, Ys>;

    #[inline]
    fn eval(self, acc: Ys) -> list::Append<Xs, Ys> {
        self.0 + acc
    }
}

impl<
    Rec: List,
    Xf: Diff,
    Yf: Diff,
    Zs: List,
> EvalOp<Zs> for Append<Xf, Yf> where
    Xf: EvalOp<Rec>,
    Yf: EvalOp<Zs, Output = Rec>,
{
    type Output = Eval<Xf, Rec>;

    #[inline]
    fn eval(self, acc: Zs) -> Eval<Xf, Rec> {
        (self.0).eval((self.1).eval(acc))
    }
}

pub trait ToList: Diff + EvalOp<list::Nil> {
    type Output;

    #[inline]
    fn list(self) -> Eval<Self, list::Nil> where
        Self: Sized,
    {
        self.eval(list::nil())
    }
}

impl<Xf: Diff + EvalOp<list::Nil>> ToList for Xf {
    type Output = Eval<Xf, list::Nil>;
}

#[cfg(test)]
mod test {
    use list::{
        List,
        self,
    };
    use list::diff::{
        Diff,
        ToDiff,
        ToList,
    };

    #[test]
    fn welp() {
        println!("{:?}", (list::nil().cons(false).cons(42u64).cons("foo").diff()
                        + list::nil().cons(false).cons(42u64).cons(vec![()]).diff()
                        + list::nil().cons(false).cons(42u64).cons(()).diff().snoc("welp")
                        + list::nil().cons("lel").cons(vec![0u8, 1u8]).cons(true).diff()).list());
    }
}
