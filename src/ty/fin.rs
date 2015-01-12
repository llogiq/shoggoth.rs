use ty::bool;
use ty::fun;
use ty::nat::{
    self,
    Nat,
};

pub struct Zero<N: Nat> where
    nat::LT: fun::Fn<(nat::Zero, N,), Out = bool::True>;
pub struct Succ<N: Nat, P: Fin<N>> where
    nat::LT: fun::Fn<(nat::Zero, N,), Out = bool::True>;

pub trait Fin<N: Nat> {}
impl<N: Nat> Fin<N> for Zero<N> where
    nat::LT: fun::Fn<(nat::Zero, N,), Out = bool::True> {}
impl<N: Nat, P: Fin<N>> Fin<nat::Succ<N>> for Succ<N, P> where
    nat::LT: fun::Fn<(nat::Zero, N,), Out = bool::True> {}
