use hlist::*;
use ty::{
    AppEval,
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
       B: Ty,
      Lx: Infer<Ty = Lens<S, A, T, B>>,
       S: Ty,
       T: Ty,
>
    Infer
for
    View<Lx>
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<S, A>;
}

impl<
       A: Ty,
       B: Ty,
      Lx: Infer<Mode = LxM, Ty = Lens<S, A, T, B>>,
     LxM,
    Get0: Tm<A>,
    Rec0: StoreLike<A, T, B, Get = Get0>,
       S: Ty,
      Sm: Tm<S>,
       T: Ty,
>
    Eval<View<Lx>>
for
    HC<Sm, HN>
where
      HC<Sm, HN>
        : AppEval<LxM, HC<S, HN>, Lx, Out = MkStore<Rec0>>,
{
    type Out = Get0;
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
      Fx: Infer<Mode = FxM, Ty = Ar1<A, B>>,
     FxM,
      Lx: Infer<Mode = LxM, Ty = Lens<S, A, T, B>>,
     LxM,
    Rec0,
    Get0,
   Set0M,
    Set0: Infer<Mode = Set0M, Ty = Ar1<B, T>>,
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
    Rec0: StoreLike<A, T, B, Get = Get0, Set = Set0>,
    HC<Rec1, HN>
        : AppEval<Set0M, HC<B, HN>, Set0, Out = Rec2>,
    HC<Sm, HN>
        : AppEval<LxM, HC<S, HN>, Lx, Out = MkStore<Rec0>>,
    HC<Get0, HN>
        : AppEval<FxM, HC<A, HN>, Fx, Out = Rec1>,
{
    type Out = Rec2;
}
