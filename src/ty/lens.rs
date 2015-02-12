use hlist::*;
use ty::{
    AppEval,
    Eval,
    Infer,
    TmPre,
    infer,
    self,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MkStore<Get, Set> where
    Set: Infer<Arity = HC<(), HN>>
{}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum View<Lx> where
    Lx: Infer<Arity = HC<(), HN>>
{}

impl<Lx> Infer for View<Lx> where
      Lx: Infer<Arity = HC<(), HN>>,
{
    type Arity = HC<(), HN>;
    type Mode = infer::mode::Constant;
}

impl<
      Lx: Infer<Arity = HC<(), HN>, Mode = LxM>,
     LxM,
    Get0,
    Set0,
      Sm,
> Eval<View<Lx>> for HC<Sm, HN> where
      HC<Sm, HN>
        : TmPre<HC<(), HN>, Out = HN>
        + AppEval<LxM, HC<(), HN>, Lx, Out = MkStore<Get0, Set0>>,
{
    type Out = Get0;
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Over<Lx: Infer> {}

impl<Lx: Infer> Infer for Over<Lx> {
    type Arity = HC<HC<(), HN>, HC<(), HN>>;
    type Mode = infer::mode::Constant;
}

impl<
      Fx: Infer<Arity = HC<(), HN>, Mode = FxM>,
     FxM,
      Lx: Infer<Arity = HC<(), HN>, Mode = LxM>,
     LxM,
    Get0,
   Set0M,
    Set0: Infer<Arity = HC<(), HN>, Mode = Set0M>,
    Rec1,
    Rec2,
      Sm,
> Eval<Over<Lx>> for HC<Fx, HC<Sm, HN>> where
    HC<Fx, HC<Sm, HN>>
        : TmPre<HC<HC<(), HN>, HC<(), HN>>, Out = HN>,
    HC<Rec1, HN>
        : AppEval<Set0M, HC<(), HN>, Set0, Out = Rec2>,
    HC<Sm, HN>
        : AppEval<LxM, HC<(), HN>, Lx, Out = MkStore<Get0, Set0>>,
    HC<Get0, HN>
        : AppEval<FxM, HC<(), HN>, Fx, Out = Rec1>,
{
    type Out = Rec2;
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Cmp<Fx, Gx> where
      Fx: Infer<Arity = HC<(), HN>>,
      Gx: Infer<Arity = HC<(), HN>>,
{}

impl<Fx, Gx> Infer for Cmp<Fx, Gx> where
      Fx: Infer<Arity = HC<(), HN>>,
      Gx: Infer<Arity = HC<(), HN>>,
{
    type Arity = HC<(), HN>;
    type Mode = infer::mode::Constant;
}

impl<
     FxM,
      Fx: Infer<Arity = HC<(), HN>, Mode = FxM>,
    Get0,
    Get1,
    Set0: Infer<Arity = HC<(), HN>>,
    Set1: Infer<Arity = HC<(), HN>>,
     GxM,
      Gx: Infer<Arity = HC<(), HN>, Mode = GxM>,
      Sm,
> Eval<Cmp<Fx, Gx>> for HC<Sm, HN> where
    HC<Sm, HN>
        : TmPre<HC<(), HN>, Out = HN>
        + AppEval<FxM, HC<(), HN>, Fx, Out = MkStore<Get0, Set0>>,
    HC<Get0, HN>
        : AppEval<GxM, HC<(), HN>, Gx, Out = MkStore<Get1, Set1>>,
{
    type Out = MkStore<Get1, ty::Cmp1<Set1, Set0>>;
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Set<Lx> where
      Lx: Infer<Arity = HC<(), HN>>,
{}

impl<Lx> Infer for Set<Lx> where
      Lx: Infer<Arity = HC<(), HN>>
{
    type Arity = HC<(), HC<(), HN>>;
    type Mode = infer::mode::Constant;
}

impl<
      Bm,
      Lx: Infer<Arity = HC<(), HN>, Mode = LxM>,
     LxM,
    Get0,
   Set0M,
    Set0: Infer<Arity = HC<(), HN>, Mode = Set0M>,
    Rec1,
      Sm,
> Eval<Set<Lx>> for HC<Bm, HC<Sm, HN>> where
    HC<Bm, HC<Sm, HN>>
        : TmPre<HC<(), HC<(), HN>>, Out = HN>,
    HC<Bm, HN>
        : AppEval<Set0M, HC<(), HN>, Set0, Out = Rec1>,
    HC<Sm, HN>
        : AppEval<LxM, HC<(), HN>, Lx, Out = MkStore<Get0, Set0>>,
{
    type Out = Rec1;
}
