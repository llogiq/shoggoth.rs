use bit::{
    _0,
    _1,
    Bit,
};
use nat::{
    IsNat,
    Pos,
    W,
    self,
};
use order;
use std;

// Fn: Succ ///////////////////////////////////////////////////////////////////

// unwrap/rewrap
impl<N: IsNat, Rec: Pos> Fn<(W<N>,)> for nat::ops::Succ where
    nat::ops::Succ: Fn<(N,), Output = Rec>
{
    type Output = W<Rec>;
    #[inline(always)]
    extern "rust-call" fn call(&self, (W(n),): (W<N>,)) -> W<Rec> {
        W(nat::ops::Succ.call((n,)))
    }
}

/// `succ(0) ==> 1`
impl Fn<(_0,)> for nat::ops::Succ {
    type Output = _1;
    #[inline(always)]
    extern "rust-call" fn call(&self, (_0 {},): (_0,)) -> _1 {
        _1
    }
}
/// `succ(1) ==> 1:0`
impl Fn<(_1,)> for nat::ops::Succ {
    type Output = (_1, _0);
    #[inline(always)]
    extern "rust-call" fn call(&self, (_1 {},): (_1,)) -> (_1, _0) {
        (_1, _0)
    }
}
/// `succ(p:0) ==> p:1`
impl<LHS: Pos> Fn<((LHS, _0),)> for nat::ops::Succ {
    type Output = (LHS, _1);
    #[inline(always)]
    extern "rust-call" fn call(&self, ((lhs, _),): ((LHS, _0),)) -> (LHS, _1) {
        (lhs, _1)
    }
}
/// `succ(p:1) ==> succ(p):0`
impl<LHS: Pos, Rec> Fn<((LHS, _1),)> for nat::ops::Succ where
    nat::ops::Succ: Fn<(LHS,), Output = Rec>,
{
    type Output = (Rec, _0);
    #[inline(always)]
    extern "rust-call" fn call(&self, ((lhs, _),): ((LHS, _1),)) -> (Rec, _0) {
        (nat::ops::Succ(lhs), _0)
    }
}

// Fn: Add ////////////////////////////////////////////////////////////////////

// unwrap/rewrap
impl<LHS: IsNat, RHS: IsNat, Rec: Pos> Fn<(W<LHS>, W<RHS>)> for nat::ops::Add where
    nat::ops::Add: Fn<(LHS, RHS), Output = Rec>
{
    type Output = W<Rec>;
    #[inline(always)]
    extern "rust-call" fn call(&self, (W(lhs), W(rhs)): (W<LHS>, W<RHS>)) -> W<Rec> {
        W(nat::ops::Add(lhs, rhs))
    }
}

/// `add(0, q) ==> q`
impl<RHS: Pos> Fn<(_0, RHS)> for nat::ops::Add {
    type Output = RHS;
    #[inline]
    extern "rust-call" fn call(&self, (_0 {}, rhs): (_0, RHS)) -> RHS {
        rhs
    }
}
/// `add(p, 0) ==> p`
impl<LHS: Pos> Fn<(LHS, _0)> for nat::ops::Add {
    type Output = LHS;
    #[inline]
    extern "rust-call" fn call(&self, (lhs, _0 {}): (LHS, _0)) -> LHS {
        lhs
    }
}
/// `add(1, 1) ==> 1:0`
impl Fn<(_1, _1)> for nat::ops::Add {
    type Output = (_1, _0);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {}, _1 {}): (_1, _1)) -> (_1, _0) {
        (_1, _0)
    }
}
/// `add(1, q:0) ==> q:1`
impl<RHS: Pos> Fn<(_1, (RHS, _0))> for nat::ops::Add {
    type Output = (RHS, _1);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {}, (rhs, _0 {})): (_1, (RHS, _0))) -> (RHS, _1) {
        (rhs, _1)
    }
}
/// `add(1, q:1) ==> succ(q):0`
impl<RHS: Pos, Rec> Fn<(_1, (RHS, _1))> for nat::ops::Add where
    nat::ops::Succ: Fn<(RHS,), Output = Rec>
{
    type Output = (Rec, _0);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {}, (rhs, _1 {})): (_1, (RHS, _1))) -> (Rec, _0) {
        (nat::ops::Succ(rhs), _0)
    }
}
/// `add(p:0, 1) ==> p:1`
impl<LHS: Pos> Fn<((LHS, _0), _1)> for nat::ops::Add {
    type Output = (LHS, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _0 {}), _1 {}): ((LHS, _0), _1)) -> (LHS, _1) {
        (lhs, _1)
    }
}
/// `add(p:0, q:0) ==> add(p, q):0`
impl<LHS: Pos, RHS: Pos, Rec> Fn<((LHS, _0), (RHS, _0))> for nat::ops::Add where
    nat::ops::Add: Fn<(LHS, RHS), Output = Rec>
{
    type Output = (Rec, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _0 {}), (rhs, _0 {})): ((LHS, _0), (RHS, _0))) -> (Rec, _0) {
        (nat::ops::Add(lhs, rhs), _0)
    }
}
/// `add(p:0, q:1) ==> add(p, q):1`
impl<LHS: Pos, RHS: Pos, Rec> Fn<((LHS, _0), (RHS, _1))> for nat::ops::Add where
    nat::ops::Add: Fn<(LHS, RHS), Output = Rec>
{
    type Output = (Rec, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _0 {}), (rhs, _1 {})): ((LHS, _0), (RHS, _1))) -> (Rec, _1) {
        (nat::ops::Add(lhs, rhs), _1)
    }
}
                   /// `add(p:1, 1) ==> succ(p):0`
impl<LHS: Pos, Rec> Fn<((LHS, _1), _1)> for nat::ops::Add where
    nat::ops::Succ: Fn<(LHS,), Output = Rec>
{
    type Output = (Rec, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _1 {}), _1 {}): ((LHS, _1), _1)) -> (Rec, _0) {
        (nat::ops::Succ(lhs), _0)
    }
}
/// `add(p:1, q:0) ==> add(p, q):1`
impl<LHS: Pos, RHS: Pos, Rec> Fn<((LHS, _1), (RHS, _0,))> for nat::ops::Add where
    nat::ops::Add: Fn<(LHS, RHS), Output = Rec>
{
    type Output = (Rec, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _1 {}), (rhs, _0 {})): ((LHS, _1), (RHS, _0))) -> (Rec, _1) {
        (nat::ops::Add(lhs, rhs), _1)
    }
}
/// `add(p:1, q:1) ==> add_carry(p, q):0`
impl<LHS: Pos, RHS: Pos, Rec> Fn<((LHS, _1), (RHS, _1,))> for nat::ops::Add where
    nat::ops::AddCarry: Fn<(LHS, RHS), Output = Rec>
{
    type Output = (Rec, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _1 {}), (rhs, _1 {})): ((LHS, _1), (RHS, _1))) -> (Rec, _0) {
        (nat::ops::AddCarry(lhs, rhs), _0)
    }
}

// Fn: AddCarry ///////////////////////////////////////////////////////////////

/// `add_carry(1, 1) ==> 1:1`
impl Fn<(_1, _1)> for nat::ops::AddCarry {
    type Output = (_1, _1);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {}, _1 {}): (_1, _1)) -> (_1, _1) {
        (_1, _1)
    }
}
/// `add_carry(1, q:0) ==> succ(q):0`
impl<RHS: Pos, Rec> Fn<(_1, (RHS, _0))> for nat::ops::AddCarry where
    nat::ops::Succ: Fn<(RHS,), Output = Rec>
{
    type Output = (Rec, _0);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {}, (rhs, _0 {})): (_1, (RHS, _0))) -> (Rec, _0) {
        (nat::ops::Succ(rhs), _0)
    }
}
/// `add_carry(1, q:1) ==> succ(q):1`
impl<RHS: Pos, Rec> Fn<(_1, (RHS, _1))> for nat::ops::AddCarry where
    nat::ops::Succ: Fn<(RHS,), Output = Rec>
{
    type Output = (Rec, _1);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {}, (rhs, _1 {})): (_1, (RHS, _1))) -> (Rec, _1) {
        (nat::ops::Succ(rhs), _1)
    }
}
/// `add_carry(p:0, 1) ==> succ(p):0`
impl<LHS: Pos, Rec> Fn<((LHS, _0), _1)> for nat::ops::AddCarry where
    nat::ops::Succ: Fn<(LHS,), Output = Rec>
{
    type Output = (Rec, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _0 {},), _1 {}): ((LHS, _0), _1)) -> (Rec, _0) {
        (nat::ops::Succ(lhs), _0)
    }
}
/// `add_carry(p:0, q:0) ==> add(p, q):1`
impl<LHS: Pos, RHS: Pos, Rec> Fn<((LHS, _0), (RHS, _0))> for nat::ops::AddCarry where
    nat::ops::Add: Fn<(LHS, RHS), Output = Rec>
{
    type Output = (Rec, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _0 {}), (rhs, _0 {})): ((LHS, _0), (RHS, _0))) -> (Rec, _1) {
        (nat::ops::Add(lhs, rhs), _1)
    }
}
/// `add_carry(p:0, q:1) ==> add_carry(p, q):0`
impl<LHS: Pos, RHS: Pos, Rec> Fn<((LHS, _0), (RHS, _1))> for nat::ops::AddCarry where
    nat::ops::AddCarry: Fn<(LHS, RHS), Output = Rec>
{
    type Output = (Rec, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _0 {}), (rhs, _1 {})): ((LHS, _0), (RHS, _1))) -> (Rec, _0) {
        (nat::ops::AddCarry(lhs, rhs), _0)
    }
}
/// `add_carry(p:1, 1) ==> succ(p):1`
impl<LHS: Pos, Rec> Fn<((LHS, _1), _1)> for nat::ops::AddCarry where
    nat::ops::Succ: Fn<(LHS,), Output = Rec>
{
    type Output = (Rec, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _1 {}), _1 {}): ((LHS, _1), _1)) -> (Rec, _1) {
        (nat::ops::Succ(lhs), _1)
    }
}
/// `add_carry(p:1, q:0) ==> add_carry(p, q):0`
impl<LHS: Pos, RHS: Pos, Rec> Fn<((LHS, _1), (RHS, _0,))> for nat::ops::AddCarry where
    nat::ops::AddCarry: Fn<(LHS, RHS), Output = Rec>
{
    type Output = (Rec, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _1 {}), (rhs, _0 {})): ((LHS, _1), (RHS, _0))) -> (Rec, _0) {
        (nat::ops::AddCarry(lhs, rhs), _0)
    }
}
/// `add_carry(p:1, q:1) ==> add_carry(p, q):1`
impl<LHS: Pos, RHS: Pos, Rec> Fn<((LHS, _1), (RHS, _1,))> for nat::ops::AddCarry where
    nat::ops::AddCarry: Fn<(LHS, RHS), Output = Rec>
{
    type Output = (Rec, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _1 {}), (rhs, _1 {})): ((LHS, _1), (RHS, _1))) -> (Rec, _1) {
        (nat::ops::AddCarry(lhs, rhs), _1)
    }
}

// Infix: Add /////////////////////////////////////////////////////////////////

// unwrap/rewrap
impl<M: IsNat, N: IsNat, Rec: IsNat> std::ops::Add<W<N>> for W<M> where
    nat::ops::Add: Fn<(M, N), Output = Rec>
{
    type Output = W<Rec>;
    #[inline]
    fn add(self, rhs: W<N>) -> W<Rec> {
        W(nat::ops::Add(self.0, rhs.0))
    }
}
