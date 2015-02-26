use numerics::bit::*;
use numerics::nat::*;
use numerics::nat::ops::*;
use std;

// Fn: Div /////////////////////////////////////////////////////////////////////

// Infix: Div //////////////////////////////////////////////////////////////////

// unwrap/rewrap
impl<M: IsNat, N: IsNat, Rec: IsNat> std::ops::Div<W<N>> for W<M> where
    Div: Fn(M, N) -> Rec
{
    type Output = W<Rec>;
    #[inline(always)]
    fn div(self, rhs: W<N>) -> W<Rec> {
        W(Div(self.0, rhs.0))
    }
}
