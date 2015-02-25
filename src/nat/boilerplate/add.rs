use bit::{
    _0,
    _1,
};
use nat::{
    IsNat,
    Pos,
    W,
};
use nat::ops::*;
use std;

// Fn: Add /////////////////////////////////////////////////////////////////////

// unwrap/rewrap
impl<LHS: IsNat, RHS: IsNat, Rec: IsNat> Fn<(W<LHS>, W<RHS>)> for Add where
    Add: Fn(LHS, RHS) -> Rec
{
    type Output = W<Rec>;
    #[inline]
    extern "rust-call" fn call(&self, (W(lhs), W(rhs)): (W<LHS>, W<RHS>)) -> W<Rec> {
        W(Add(lhs, rhs))
    }
}

/// `add(0, q) ==> q`
impl<RHS: Pos> Fn<(_0, RHS)> for Add {
    type Output = RHS;
    #[inline]
    extern "rust-call" fn call(&self, (_0 {}, rhs): (_0, RHS)) -> RHS {
        rhs
    }
}
/// `add(p, 0) ==> p`
impl<LHS: Pos> Fn<(LHS, _0)> for Add {
    type Output = LHS;
    #[inline]
    extern "rust-call" fn call(&self, (lhs, _0 {}): (LHS, _0)) -> LHS {
        lhs
    }
}
/// `add(1, 1) ==> 1:0`
impl Fn<(_1, _1)> for Add {
    type Output = (_1, _0);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {}, _1 {}): (_1, _1)) -> (_1, _0) {
        (_1, _0)
    }
}
/// `add(1, q:0) ==> q:1`
impl<RHS: Pos> Fn<(_1, (RHS, _0))> for Add {
    type Output = (RHS, _1);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {}, (rhs, _0 {})): (_1, (RHS, _0))) -> (RHS, _1) {
        (rhs, _1)
    }
}
/// `add(1, q:1) ==> succ(q):0`
impl<RHS: Pos, Rec> Fn<(_1, (RHS, _1))> for Add where
    Succ: Fn(RHS) -> Rec
{
    type Output = (Rec, _0);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {}, (rhs, _1 {})): (_1, (RHS, _1))) -> (Rec, _0) {
        (Succ(rhs), _0)
    }
}
/// `add(p:0, 1) ==> p:1`
impl<LHS: Pos> Fn<((LHS, _0), _1)> for Add {
    type Output = (LHS, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _0 {}), _1 {}): ((LHS, _0), _1)) -> (LHS, _1) {
        (lhs, _1)
    }
}
/// `add(p:0, q:0) ==> add(p, q):0`
impl<LHS: Pos, RHS: Pos, Rec> Fn<((LHS, _0), (RHS, _0))> for Add where
    Add: Fn(LHS, RHS) -> Rec
{
    type Output = (Rec, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _0 {}), (rhs, _0 {})): ((LHS, _0), (RHS, _0))) -> (Rec, _0) {
        (Add(lhs, rhs), _0)
    }
}
/// `add(p:0, q:1) ==> add(p, q):1`
impl<LHS: Pos, RHS: Pos, Rec> Fn<((LHS, _0), (RHS, _1))> for Add where
    Add: Fn(LHS, RHS) -> Rec
{
    type Output = (Rec, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _0 {}), (rhs, _1 {})): ((LHS, _0), (RHS, _1))) -> (Rec, _1) {
        (Add(lhs, rhs), _1)
    }
}
/// `add(p:1, 1) ==> succ(p):0`
impl<LHS: Pos, Rec> Fn<((LHS, _1), _1)> for Add where
    Succ: Fn(LHS) -> Rec
{
    type Output = (Rec, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _1 {}), _1 {}): ((LHS, _1), _1)) -> (Rec, _0) {
        (Succ(lhs), _0)
    }
}
/// `add(p:1, q:0) ==> add(p, q):1`
impl<LHS: Pos, RHS: Pos, Rec> Fn<((LHS, _1), (RHS, _0,))> for Add where
    Add: Fn(LHS, RHS) -> Rec
{
    type Output = (Rec, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _1 {}), (rhs, _0 {})): ((LHS, _1), (RHS, _0))) -> (Rec, _1) {
        (Add(lhs, rhs), _1)
    }
}
/// `add(p:1, q:1) ==> add_carry(p, q):0`
impl<LHS: Pos, RHS: Pos, Rec> Fn<((LHS, _1), (RHS, _1,))> for Add where
    AddCarry: Fn(LHS, RHS) -> Rec
{
    type Output = (Rec, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _1 {}), (rhs, _1 {})): ((LHS, _1), (RHS, _1))) -> (Rec, _0) {
        (AddCarry(lhs, rhs), _0)
    }
}

// Infix: Add //////////////////////////////////////////////////////////////////

// unwrap/rewrap
impl<M: IsNat, N: IsNat, Rec: IsNat> std::ops::Add<W<N>> for W<M> where
    Add: Fn(M, N) -> Rec
{
    type Output = W<Rec>;
    #[inline]
    fn add(self, rhs: W<N>) -> W<Rec> {
        W(Add(self.0, rhs.0))
    }
}
