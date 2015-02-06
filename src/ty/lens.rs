use hlist::*;
use ty::{
    AppEval,
    Ar,
    Ar1,
    Eval,
    Infer,
    IsArrow,
    Tm,
    Ty,
    infer,
    self,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Store<A: Ty, T: Ty, B: Ty = A> {}

impl<A: Ty, B: Ty, T: Ty> Ty for Store<A, T, B> {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MkStore<Get, Set> where
     Set: Infer,
    <Set as Infer>::Ty
        : IsArrow,
{}

impl<
       A: Ty,
       B: Ty,
     Get: Tm<A>,
     Set: Infer<Ty = Ar1<B, T>>,
       T: Ty,
> Tm<Store<A, T, B>> for MkStore<Get, Set> {}

pub type Lens<S, A, T=S, B=A> = Ar1<S, Store<A, T, B>>;

#[rustc_on_unimplemented = "`{Self}` is not a valid type-level lens"]
pub trait IsLens {
    type S: Ty;
    type A: Ty;
    type T: Ty;
    type B: Ty;
}

impl<S: Ty, A: Ty, T: Ty, B: Ty> IsLens for Lens<S, A, T, B> {
    type S = S;
    type A = A;
    type T = T;
    type B = B;
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum View<Lx> where
      Lx: Infer,
   <Lx as Infer>::Ty: IsLens,
{}

impl<
       A: Ty,
       B: Ty,
      Lx: Infer<Ty = Lens<S, A, T, B>>,
       S: Ty,
       T: Ty,
> Infer for View<Lx> {
    type Mode = infer::mode::Constant;
    type Ty = Ar1<S, A>;
}

impl<
       A: Ty,
       B: Ty,
      Lx: Infer<Mode = LxM, Ty = Lens<S, A, T, B>>,
     LxM,
    Get0: Tm<A>,
    Set0,
       S: Ty,
      Sm: Tm<S>,
       T: Ty,
> Eval<View<Lx>> for HC<Sm, HN> where
      HC<Sm, HN>
        : AppEval<LxM, HC<S, HN>, Lx, Out = MkStore<Get0, Set0>>,
{
    type Out = Get0;
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Over<Lx> where
      Lx: Infer,
   <Lx as Infer>::Ty: IsLens,
{}

impl<
       A: Ty,
       B: Ty,
      Lx: Infer,
       S: Ty,
       T: Ty,
> Infer for Over<Lx> where
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
    Get0,
   Set0M,
    Set0: Infer<Mode = Set0M, Ty = Ar1<B, T>>,
    Rec1,
    Rec2: Tm<T>,
       S: Ty,
      Sm: Tm<S>,
       T: Ty,
> Eval<Over<Lx>> for HC<Fx, HC<Sm, HN>> where
    HC<Rec1, HN>
        : AppEval<Set0M, HC<B, HN>, Set0, Out = Rec2>,
    HC<Sm, HN>
        : AppEval<LxM, HC<S, HN>, Lx, Out = MkStore<Get0, Set0>>,
    HC<Get0, HN>
        : AppEval<FxM, HC<A, HN>, Fx, Out = Rec1>,
{
    type Out = Rec2;
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Cmp<Fx, Gx> where
      Fx: Infer,
   <Fx as Infer>::Ty
        : IsLens,
      Gx: Infer,
   <Gx as Infer>::Ty
        : IsLens< S = <<Fx as Infer>::Ty as IsLens>::A
                , T = <<Fx as Infer>::Ty as IsLens>::B
                >,
{}

impl<
       A: Ty,
       B: Ty,
      Fx: Infer<Ty = Lens<S, A, T, B>>,
      Gx: Infer<Ty = Lens<A, U, B, V>>,
       S: Ty,
       T: Ty,
       U: Ty,
       V: Ty,
> Infer for Cmp<Fx, Gx> {
    type Mode = infer::mode::Constant;
    type Ty = Lens<S, U, T, V>;
}

impl<
       A: Ty,
       B: Ty,
     FxM,
      Fx: Infer<Mode = FxM, Ty = Lens<S, A, T, B>>,
    Get0,
    Get1: Tm<U>,
    Set0: Infer<Ty = Ar1<B, T>>,
    Set1: Infer<Ty = Ar1<V, B>>,
     GxM,
      Gx: Infer<Mode = GxM, Ty = Lens<A, U, B, V>>,
       S: Ty,
      Sm: Tm<S>,
       T: Ty,
       U: Ty,
       V: Ty,
> Eval<Cmp<Fx, Gx>> for HC<Sm, HN> where
    HC<Sm, HN>
        : AppEval<FxM, HC<S, HN>, Fx, Out = MkStore<Get0, Set0>>,
    HC<Get0, HN>
        : AppEval<GxM, HC<A, HN>, Gx, Out = MkStore<Get1, Set1>>,
{
    type Out = MkStore<Get1, ty::Cmp1<Set1, Set0>>;
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Set<Lx> where
      Lx: Infer,
   <Lx as Infer>::Ty: IsLens,
{}

impl<
       A: Ty,
       B: Ty,
      Lx: Infer,
       S: Ty,
       T: Ty,
> Infer for Set<Lx> where
   <Lx as Infer>::Ty
        : IsLens<S = S, A = A, T = T, B = B>,
{
    type Mode = infer::mode::Constant;
    type Ty = Ar<HC<B, HC<S, HN>>, T>;
}

impl<
       A: Ty,
       B: Ty,
      Bm: Tm<B>,
      Lx: Infer<Mode = LxM, Ty = Lens<S, A, T, B>>,
     LxM,
    Get0,
   Set0M,
    Set0: Infer<Mode = Set0M, Ty = Ar1<B, T>>,
    Rec1: Tm<T>,
       S: Ty,
      Sm: Tm<S>,
       T: Ty,
> Eval<Set<Lx>> for HC<Bm, HC<Sm, HN>> where
    HC<Bm, HN>
        : AppEval<Set0M, HC<B, HN>, Set0, Out = Rec1>,
    HC<Sm, HN>
        : AppEval<LxM, HC<S, HN>, Lx, Out = MkStore<Get0, Set0>>,
{
    type Out = Rec1;
}
