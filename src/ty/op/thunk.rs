use hlist::*;
use ty::{
    Infer,
    TmPrefix,
    Ty,
    infer,
};
use ty::op::{
    Ar,
    IsArrow,
};



/// Curried type-level operations
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Thunk<Fx, Xs>
where
      Fx: Infer,
      Xs: HList,
      Xs: TmPrefix<<<Fx as Infer>::Ty as IsArrow>::Dom>,
    <Fx as Infer>::Ty
        : IsArrow,
{}

impl<
       C: Ty,
       D: Ty + HList,
      Ds: Ty + HList,
      Fx: Infer<Ty = Ar<D, C>>,
      Xs: HList,
>
    Infer
for
    Thunk<Fx, Xs>
where
      Xs: TmPrefix<D, Out = Ds>,
{
    type Mode = infer::mode::Thunk;
    type Ty = Ar<Ds, C>;
}



pub trait
    ProjDoms
where
    Self: HList,
{
    type Out: Ty + HList;
}

impl
    ProjDoms
for
    HN
{
    type Out = HN;
}

impl<
       D,
      Fx: Infer,
     Fxs: HList,
    Rec0,
    Rec1: Ty + HList,
>
    ProjDoms
for
    HC<Fx, Fxs>
where
       D: Append<Rec0, Out = Rec1>,
     Fxs: ProjDoms<Out = Rec0>,
    <Fx as Infer>::Ty
        : IsArrow<Dom = D>,
{
    type Out = Rec1;
}



pub trait
    ProjCods
where
    Self: HList,
{
    type Out: Ty + HList;
}

impl
    ProjCods
for
    HN
{
    type Out = HN;
}

impl<
      Fx: Infer,
     Fxs: HList,
>
    ProjCods
for
    HC<Fx, Fxs>
where
    <Fx as Infer>::Ty: IsArrow,
     Fxs: ProjCods,
{
    type Out =
        HC<
            <<Fx as Infer>::Ty as IsArrow>::Cod,
            <Fxs as ProjCods>::Out,
        >;
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
    Cmp<Fxs, Gx>
where
     Fxs: ProjCods,
      Gx: Infer,
   <Gx as Infer>::Ty
        : IsArrow<Dom = <Fxs as ProjCods>::Out>,
{}

impl<
    Fxs,
    Gx: Infer,
>
    Infer
for
    Cmp<Fxs, Gx>
where
     Fxs: ProjDoms + ProjCods,
   <Gx as Infer>::Ty
        : IsArrow<Dom = <Fxs as ProjCods>::Out>,
{
    type Mode = infer::mode::Constant;
    type Ty =
        Ar<
            <Fxs as ProjDoms>::Out,
            <<Gx as Infer>::Ty as IsArrow>::Cod,
        >;
}



#[cfg(test)]
mod test {
    use hlist::*;
    use ty::*;

    #[test]
    fn proj_cods() {
        let x0: Witness<
            <HC<And,       // [Bool, Bool] -> Bool
             HC<nat::Succ, // [Nat] -> Nat
             HN>>
            as ProjCods>::Out
        > = Witness;
        let x1: Witness<
             HC<Bool,
             HC<nat::Nat,
             HN>>
        > = Witness;
        x0 == x1;
    }

    #[test]
    fn proj_doms() {
        let x0: Witness<
            <HC<And,       // [Bool, Bool] -> Bool
             HC<nat::Succ, // [Nat] -> Nat
             HN>>
            as ProjDoms>::Out
        > = Witness;
        let x1: Witness<
             HC<Bool,
             HC<Bool,
             HC<nat::Nat,
             HN>>>
        > = Witness;
        x0 == x1;
    }

    #[test]
    fn cmp() {
        fn aux<Fx: Tm<Ar<HC<Bool, HC<Bool, HN>>, Bool>>>() {}
        aux::<
            Cmp<
                HC<Not,
                HC<Not,
                HN>>,
                And,
            >
        >();
    }
}

