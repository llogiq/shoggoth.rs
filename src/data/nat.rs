use std::marker::*;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct O<X, Xs>(pub PhantomData<(X, Xs)>);

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct I<X, Xs>(pub PhantomData<(X, Xs)>);

// pub trait AllPos: MarkerTrait + list::List {}
// impl AllPos for list::Nil {}
// impl<X: Pos, Xs: AllPos> AllPos for list::Cons<X, Xs> {}

// pub trait Pos: MarkerTrait {}
// impl Pos for _1 {}
// impl<X: Pos, Xs: AllPos> Pos for O<X, Xs> {}
// impl<X: Pos, Xs: AllPos> Pos for I<X, Xs> {}
