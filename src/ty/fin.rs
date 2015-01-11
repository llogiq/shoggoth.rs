use ty::fun;
use ty::nat::{
    LT,
    Nat,
    S,
    Z,
};

pub struct FZ<N: Nat> where LT: fun::Fn<(Z, N,)>;
pub struct FS<N: Nat, P: Fin<N>> where LT: fun::Fn<(Z, N,)>;

pub trait Fin<N: Nat> {}
impl<N: Nat> Fin<N> for FZ<N> where LT: fun::Fn<(Z, N,)> {}
impl<N: Nat, P: Fin<N>> Fin<S<N>> for FS<N, P> where LT: fun::Fn<(Z, N,)> {}

