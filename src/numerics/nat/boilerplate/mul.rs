use nat::{
    IsNat,
    W,
};
use nat::ops::*;
use std;

// Fn: Mul /////////////////////////////////////////////////////////////////////

// Infix: Mul //////////////////////////////////////////////////////////////////

// unwrap/rewrap
impl<M: IsNat, N: IsNat, Rec: IsNat> std::ops::Mul<W<N>> for W<M> where
    Mul: Fn(M, N) -> Rec
{
    type Output = W<Rec>;
    #[inline(always)]
    fn mul(self, rhs: W<N>) -> W<Rec> {
        W(Mul(self.0, rhs.0))
    }
}
