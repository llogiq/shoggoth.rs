use hlist::*;
use ty::{
    Ar,
    Eval,
    Infer,
    IsArrow,
    Tm,
    TmExt,
    Ty,
    infer,
};

/// Project and flatten the domains of operations in an HList
#[doc(hidden)]
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
       D: Prepend<Rec0, Out = Rec1>,
     Fxs: ProjDoms<Out = Rec0>,
    <Fx as Infer>::Ty
        : IsArrow<Dom = D>,
{
    type Out = Rec1;
}



/// Project the codomains of operations in an HList
#[doc(hidden)]
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



/// Type-level operation composition (polyadic)
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



#[doc(hidden)]
pub trait
    AppMany<Fxs>
where
     Fxs: HList,
    Self: HList,
{
    type Out: HList;
}

impl
    AppMany<HN>
for
    HN
{
    type Out = HN;
}

impl<
    Args: Tm<D> + HList,
    Rest: HList,
       C: Ty,
       D: Ty    + HList,
      Fx: Infer<Ty = Ar<D, C>>,
     Fxs: HList,
   Input: HList,
     Rec,
>
    AppMany<HC<Fx, Fxs>>
for
    Input
where
    Args: Eval<Fx, Out = Rec>,
   Input: TmExt<D, Out = Args, Ext = Rest>,
    Rest: AppMany<Fxs>,
{
    type Out =
        HC<
            Rec,
            <Rest as AppMany<Fxs>>::Out,
        >;
}



impl<
    FxsD: Ty       + HList,
    FxsC: Ty       + HList,
     GxC: Ty,
     Fxs,
      Gx: Infer<Ty = Ar<FxsC, GxC>>,
   Input: Tm<FxsD> + HList,
    Rec0: Tm<FxsC>,
    Rec1: Tm<GxC>,
>
    Eval<Cmp<Fxs, Gx>>
for
    Input
where
     Fxs: ProjDoms<Out = FxsD> + ProjCods<Out = FxsC>,
   Input: AppMany<Fxs, Out = Rec0>,
    Rec0: Eval<Gx, Out = Rec1>,
{
    type Out = Rec1;
}



// Alias for binary composition
pub type Cmp1<Gx, Fx> = Cmp<HC<Gx, HN>, Fx>;



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
        fn aux<Fx: Tm<Ar<HC<Bool, HC<Bool, HC<Bool, HN>>>, Bool>>>() {}
        aux::<
            Cmp<
                HC<Not,
                HC<Or,
                HN>>,
                And,
            >
        >();
        let nand:
            Witness<
                Ap<
                    Cmp<
                        HC<And,
                        HN>,
                        Not,
                    >,
                    HC<FF,
                    HC<FF,
                    HN>>
                >
            > = Witness;
        let res:
            Witness<
                TT
            > = Witness;
        nand == res;
    }
}
