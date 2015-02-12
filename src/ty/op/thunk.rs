use ty::{
    Infer,
    TmPre,
    infer,
};

/// Curried type-level operations
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Thunk<Fx, Xs> where
      Fx: Infer,
      Xs: TmPre<<Fx as Infer>::Arity>,
{}

impl<
       D,
      Ds,
      Fx: Infer<Arity = D>,
      Xs,
> Infer for Thunk<Fx, Xs> where
      Xs: TmPre<D, Out = Ds>,
{
    type Arity = Ds;
    type Mode = infer::mode::Thunk;
}
