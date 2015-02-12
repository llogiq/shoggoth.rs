use hlist::*;
use ty::{
    Infer,
    TmPre,
    infer,
};
use ty::op::{
    Eval,
    Thunk,
};

/// Partially apply a thunk to an argument or evaluate a constant
/// (i.e., operation symbol)
#[rustc_on_unimplemented = "`{Fx}` cannot be applied to `{Self}`"]
pub trait AppEval<M, FxDTy, Fx>: TmPre<FxDTy> where
      Fx: Infer<Arity = FxDTy>,
       M: infer::mode::Mode,
{
    type Out;
}

impl<
    Args,
      Cx: Infer<Arity = CxDTy>,
   CxDTy,
> AppEval<
    infer::mode::Constant,
    CxDTy,
    Cx
> for Args where
    Args: AppEval<infer::mode::Thunk, CxDTy, Thunk<Cx, HN>>,
    Args: TmPre<CxDTy>,
{
    type Out = <Args as AppEval<
        infer::mode::Thunk,
        CxDTy,
        Thunk<Cx, HN>>
    >::Out;
}

impl<
      Fx: Infer<Arity = FxDTy>,
   FxDTy,
      Xs,
> AppEval<
    infer::mode::Thunk,
    HN,
    Thunk<Fx, Xs>,
> for HN where
    Thunk<Fx, Xs>
        : Infer<Arity = HN>,
      Xs: Eval<Fx>,        
      Xs: TmPre<FxDTy, Out = HN>,
{
    type Out = <Xs as Eval<Fx>>::Out;
}

impl<
      Tx: Infer<Arity = HC<TxDHTy, TxDTTy>>,
  TxDHTy,
  TxDTTy,
> AppEval<
    infer::mode::Thunk,
    HC<TxDHTy, TxDTTy>,
    Tx
> for HN {
    type Out = Tx;
}

impl<
     ArgsHTm,
     ArgsTTm,
          Fx: Infer<Arity = HC<FxDHTy, FxDTTy>>,
      FxDHTy,
      FxDTTy,
      TxDHTy,
      TxDTTy,
          Xs,
> AppEval<
    infer::mode::Thunk,
    HC<TxDHTy, TxDTTy>,
    Thunk<Fx, Xs>
> for HC<ArgsHTm, ArgsTTm> where
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
    type Out = <ArgsTTm as AppEval<
        infer::mode::Thunk,
        TxDTTy,
        Thunk<Fx, HS<Xs, ArgsHTm>>>
    >::Out;
}
