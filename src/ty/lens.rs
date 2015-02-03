use hlist::*;
use ty::{
    Ar,
    Ar1,
    Eval,
    Infer,
    Tm,
    Ty,
    infer,
};



pub trait
    StoreLike<A, T, B=A>
{
    type Get: Tm<A>;
    type Set: Tm<Ar1<B, T>>;
}



#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Store<A, T, B=A>
where
       A: Ty,
       B: Ty,
       T: Ty,
{}

impl<
       A: Ty,
       B: Ty,
       T: Ty,
>
    Ty
for
    Store<A, T, B>
{}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    MkStore<Sx>
{}

impl<
       A: Ty,
       B: Ty,
       T: Ty,
      Sx: StoreLike<A, T, B>
>
    Tm<Store<A, T, B>>
for
    MkStore<Sx>
{}



pub type Lens<S, A, T=S, B=A> = Ar1<S, Store<A, T, B>>;



pub trait
    IsLens
{
    type S: Ty;
    type A: Ty;
    type T: Ty;
    type B: Ty;
}

impl<
       S: Ty,
       A: Ty,
       T: Ty,
       B: Ty,
>
    IsLens
for
    Lens<S, A, T, B>
{
    type S = S;
    type A = A;
    type T = T;
    type B = B;
}



#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    View<Lx>
where
      Lx: Infer,
   <Lx as Infer>::Ty: IsLens,
{}

impl<
       A: Ty,
      Lx: Infer,
       S: Ty,
>
    Infer
for
    View<Lx>
where
  <Lx as Infer>::Ty: IsLens<S = S, A = A>,
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<S, A>;
}

impl<
       A: Ty,
       B: Ty,
      Lx: Infer<Ty = Lens<S, A, T, B>>,
     Rec: StoreLike<A, T, B>,
       S: Ty,
      Sm: Tm<S>,
       T: Ty,
>
    Eval<View<Lx>>
for
    HC<Sm, HN>
where
      HC<Sm, HN>: Eval<Lx, Out = MkStore<Rec>>,
{
    type Out = <Rec as StoreLike<A, T, B>>::Get;
}



#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Over<Lx>
where
      Lx: Infer,
   <Lx as Infer>::Ty: IsLens,
{}

impl<
       A: Ty,
       B: Ty,
      Lx: Infer,
       S: Ty,
       T: Ty,
>
    Infer
for
    Over<Lx>
where
   <Lx as Infer>::Ty
        : IsLens<S = S, A = A, T = T, B = B>,
{
    type Mode = infer::mode::Constant;
    type Ty = Ar<HC<Ar1<A, B>, HC<S, HN>>, T>;
}

impl<
       A: Ty,
       B: Ty,
      Fx: Infer<Ty = Ar1<A, B>>,
      Lx: Infer<Ty = Lens<S, A, T, B>>,
    Rec0: StoreLike<A, T, B>,
    Rec1,
    Rec2: Tm<T>,
       S: Ty,
      Sm: Tm<S>,
       T: Ty,
>
    Eval<Over<Lx>>
for
    HC<Fx, HC<Sm, HN>>
where
   HC<Rec1, HN>
        : Eval<<Rec0 as StoreLike<A, T, B>>::Set, Out = Rec2>,
   HC<<Rec0 as StoreLike<A, T, B>>::Get, HN>
        : Eval<Fx, Out = Rec1>,
   HC<Sm, HN>
        : Eval<Lx, Out = MkStore<Rec0>>,
{
    type Out = Rec2;
}
