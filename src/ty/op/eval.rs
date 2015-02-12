use hlist::*;
use ty::{
    Infer,
    TmPre,
    infer,
};

/// Interpret constants (i.e., operation symbols) at a given input
/// (i.e., `Self`)
#[rustc_on_unimplemented = "`{Fx}` cannot be evaluated at `{Self}`"]
pub trait Eval<Fx>
    : TmPre<<Fx as Infer>::Arity, Out = HN>
where
    Fx: Infer<Mode = infer::mode::Constant>
{
    type Out;
}

pub trait Eval1<Fx: Infer> where
    HC<Self, HN>
        : Eval<Fx>,
{
    type Out;
}

impl<Fx: Infer, Xs> Eval1<Fx> for Xs where
    HC<Xs, HN>
        : Eval<Fx>,
{
    type Out = <HC<Xs, HN> as Eval<Fx>>::Out;
}
