use hlist::*;
use ty::{
    AppEval,
    Eval,
    Infer,
    TmExt,
    TmPre,
    infer,
};

/// Project and flatten the domains of operations in an HList
pub trait ProjDoms {
    type Out;
}

impl ProjDoms for HN {
    type Out = HN;
}

impl<
       D,
      Fx: Infer<Arity = D>,
     Fxs,
    Rec0,
    Rec1,
> ProjDoms for HC<Fx, Fxs> where
       D: Prepend<Rec0, Out = Rec1>,
     Fxs: ProjDoms<Out = Rec0>,
{
    type Out = Rec1;
}

/// Project the codomains of operations in an HList
pub trait ProjCods {
    type Out;
}

impl ProjCods for HN {
    type Out = HN;
}

impl<
      Fx: Infer,
     Fxs,
> ProjCods for HC<Fx, Fxs> where
     Fxs: ProjCods,
{
    type Out = HC<
        (),
        <Fxs as ProjCods>::Out,
    >;
}

/// Type-level operation composition (polyadic)
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Cmp<Fxs, Gx> where
     Fxs: ProjCods,
      Gx: Infer<Arity = <Fxs as ProjCods>::Out>,
{}

impl<Fxs, Gx> Infer for Cmp<Fxs, Gx> where
     Fxs: ProjDoms + ProjCods,
      Gx: Infer<Arity = <Fxs as ProjCods>::Out>,
{
    type Arity = <Fxs as ProjDoms>::Out;
    type Mode = infer::mode::Constant;
}

#[rustc_on_unimplemented = "`{Fxs}` cannot be applied to `{Self}`"]
pub trait AppMany<Fxs> {
    type Out;
}

impl AppMany<HN> for HN {
    type Out = HN;
}

impl<
    Args,
    Rest,
       D,
      Fx: Infer<Arity = D, Mode = FxM>,
     FxM: infer::mode::Mode,
     Fxs,
   Input,
     Rec,
> AppMany<HC<Fx, Fxs>> for Input where
    Args: AppEval<FxM, D, Fx, Out = Rec>,
   Input: TmExt<D, Out = Args, Ext = Rest>,
    Rest: AppMany<Fxs>,
{
    type Out = HC<Rec, <Rest as AppMany<Fxs>>::Out>;
}

impl<
    FxsD,
    FxsC,
     Fxs,
      Gx: Infer<Arity = FxsC, Mode = GxM>,
     GxM: infer::mode::Mode,
   Input: TmPre<FxsD, Out = HN>,
    Rec0,
    Rec1,
> Eval<Cmp<Fxs, Gx>> for Input where
     Fxs: ProjDoms<Out = FxsD> + ProjCods<Out = FxsC>,
   Input: AppMany<Fxs, Out = Rec0>,
    Rec0: AppEval<GxM, FxsC, Gx, Out = Rec1>,
{
    type Out = Rec1;
}

// Alias for binary composition
pub type Cmp1<Gx, Fx> = Cmp<HC<Gx, HN>, Fx>;

#[cfg(test)]
mod test {
    use hlist::*;
    use ty::*;

    // #[test]
    // fn proj_cods() {
    //     let x0 = Witness::<
    //         <HC<And,
    //          HC<nat::Succ,
    //          HN>>
    //         as ProjCods>::Out>;
    //     let x1 = Witness::<
    //          HC<(),
    //          HC<(),
    //          HN>>>;
    //     x0 == x1;
    // }

    // #[test]
    // fn proj_doms() {
    //     let x0 = Witness::<
    //         <HC<And,       // [Bool, Bool] -> Bool
    //          HC<nat::Succ, // [Nat] -> Nat
    //          HN>>
    //         as ProjDoms>::Out>;
    //     let x1 = Witness::<
    //          HC<Bool,
    //          HC<Bool,
    //          HC<nat::Nat,
    //          HN>>>>;
    //     x0 == x1;
    // }

    #[test]
    fn cmp() {
        fn aux<Fx>() {}
        aux::<Cmp<HC<Not,
                  HC<Or,
                  HN>>,
                  And>>();
        let nand = Witness::<Ap<Cmp<HC<And, HN>, Not>,
                                HC<FF,
                                HC<FF,
                                HN>>>>;
        let res = Witness::<TT>;
        nand == res;
    }
}
