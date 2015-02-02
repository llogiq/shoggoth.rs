use hlist::*;
use ty::{
    Infer,
    TmPre,
    Ty,
    infer,
};
use ty::op::{
    Ar,
    Eval,
    IsArrow,
    Thunk,
};

/// Partially apply a thunk to an argument or evaluate a constant
/// (i.e., operation symbol)
pub trait
    AppEval<
        M,
        FxDTy,
        Fx,
    >
where
      Fx: Infer,
       M: infer::mode::Mode,
    Self: HList,
    Self:
        TmPre<
            <<Fx as Infer>::Ty as IsArrow>::Dom
        >,
    <Fx as Infer>::Ty
        : IsArrow<Dom = FxDTy>,
{
    // FIXME: should probably put a bound on Out
    type Out;
}

impl<
    Args: HList,
      Cx: Infer<Ty = Ar<CxDTy, CxCTy>>,
   CxCTy: Ty,
   CxDTy: Ty + HList,
>
    AppEval<
        infer::mode::Constant,
        CxDTy,
        Cx,
    >
for
    Args
where
    Args:
        AppEval<
            infer::mode::Thunk,
            CxDTy,
            Thunk<Cx, HN>
        >,
    Args: TmPre<CxDTy>,
{
    type Out =
        <Args as
            AppEval<
                infer::mode::Thunk,
                CxDTy,
                Thunk<Cx, HN>,
            >
        >::Out;
}

impl<
      Fx: Infer<Ty = Ar<FxDTy, TxCTy>>,
   FxDTy: Ty + HList,
   TxCTy: Ty,
      Xs: HList,
>
    AppEval<
        infer::mode::Thunk,
        HN,
        Thunk<Fx, Xs>,
    >
for
    HN
where
    Thunk<Fx, Xs>
        : Infer<Ty = Ar<HN, TxCTy>>,
      Xs: Eval<Fx>,        
      Xs: TmPre<FxDTy, Out = HN>,
{
    type Out = <Xs as Eval<Fx>>::Out;
}

impl<
      Tx: Infer<Ty = Ar<HC<TxDHTy, TxDTTy>, TxCTy>>,
   TxCTy: Ty,
  TxDHTy: Ty,
  TxDTTy: Ty + HList,
>
    AppEval<
        infer::mode::Thunk,
        HC<TxDHTy, TxDTTy>,
        Tx,
    >
for
    HN
{
    type Out = Tx;
}

impl<
     ArgsHTm,
     ArgsTTm: HList,
          Fx: Infer<Ty = Ar<HC<FxDHTy, FxDTTy>, TxCTy>>,
      FxDHTy: Ty,
      FxDTTy: Ty + HList,
       TxCTy: Ty,
      TxDHTy: Ty,
      TxDTTy: Ty + HList,
          Xs: HList,
>
    AppEval<
        infer::mode::Thunk,
        HC<TxDHTy, TxDTTy>,
        Thunk<Fx, Xs>
    >
for
    HC<ArgsHTm, ArgsTTm>
where
      Xs: TmPre<HC<FxDHTy, FxDTTy>, Out = HC<TxDHTy, TxDTTy>>,
      Xs: Snoc<ArgsHTm>,
    ArgsTTm
        : AppEval<
            infer::mode::Thunk,
            TxDTTy,
            Thunk<Fx, HS<Xs, ArgsHTm>>
        >,
    HC<ArgsHTm, ArgsTTm>
        : TmPre<HC<TxDHTy, TxDTTy>>,
{
    type Out =
        <ArgsTTm as AppEval<
            infer::mode::Thunk,
            TxDTTy,
            Thunk<Fx, HS<Xs, ArgsHTm>>
        >>::Out;
}
