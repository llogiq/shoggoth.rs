pub use self::apply::{
    AppEval,
};
pub use self::compose::{
    Cmp,
    Cmp1,
    ProjCods,
    ProjDoms,
};
pub use self::eval::{
    Eval,
    Eval1,
};
pub use self::thunk::{
    Thunk,
};
use hlist::*;
use ty::{
    Infer,
    infer,
};

mod apply;
mod compose;
mod eval;
mod thunk;

/// Alias for partially applying terms of arrow types to many
/// arguments
pub type Ap<Fx, Xs> = <Xs as AppEval<
    <Fx as Infer>::Mode,
    <Fx as Infer>::Arity,
     Fx>
>::Out;

/// Alias for partially applying terms of arrow types to zero
/// arguments
pub type Ap0<Fx> = Ap<Fx, HN>;

/// Alias for partially applying terms of arrow types to a single
/// argument
pub type Ap1<Fx, X> = Ap<Fx, HC<X, HN>>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Id {}

impl Infer for Id {
    type Arity = HC<(), HN>;
    type Mode = infer::mode::Constant;
}

impl<M> Eval<Id> for HC<M, HN> {
    type Out = M;
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Const {}

impl Infer for Const {
    type Arity = HC<(), HC<(), HN>>;
    type Mode = infer::mode::Constant;
}

impl<M, N> Eval<Const> for HC<M, HC<N, HN>> {
    type Out = M;
}
