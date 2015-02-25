use bit::{
    _0,
    _1,
    Bit,
};
use nat::{
    IsNat,
    Pos,
    W,
};
use nat::ops::*;
use order;
use std;

// Fn: Succ ////////////////////////////////////////////////////////////////////

// unwrap/rewrap
impl<N: IsNat, Rec: Pos> Fn<(W<N>,)> for Succ where
    Succ: Fn(N) -> Rec
{
    type Output = W<Rec>;
    #[inline]
    extern "rust-call" fn call(&self, (W(n),): (W<N>,)) -> W<Rec> {
        W(Succ.call((n,)))
    }
}

/// `succ(0) ==> 1`
impl Fn<(_0,)> for Succ {
    type Output = _1;
    #[inline]
    extern "rust-call" fn call(&self, (_0 {},): (_0,)) -> _1 {
        _1
    }
}
/// `succ(1) ==> 1:0`
impl Fn<(_1,)> for Succ {
    type Output = (_1, _0);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {},): (_1,)) -> (_1, _0) {
        (_1, _0)
    }
}
/// `succ(p:0) ==> p:1`
impl<LHS: Pos> Fn<((LHS, _0),)> for Succ {
    type Output = (LHS, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _),): ((LHS, _0),)) -> (LHS, _1) {
        (lhs, _1)
    }
}
/// `succ(p:1) ==> succ(p):0`
impl<LHS: Pos, Rec> Fn<((LHS, _1),)> for Succ where
    Succ: Fn(LHS) -> Rec
{
    type Output = (Rec, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _),): ((LHS, _1),)) -> (Rec, _0) {
        (Succ(lhs), _0)
    }
}

// Fn: Add /////////////////////////////////////////////////////////////////////

// unwrap/rewrap
impl<LHS: IsNat, RHS: IsNat, Rec: Pos> Fn<(W<LHS>, W<RHS>)> for Add where
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

// Fn: AddCarry ////////////////////////////////////////////////////////////////

/// `add_carry(1, 1) ==> 1:1`
impl Fn<(_1, _1)> for AddCarry {
    type Output = (_1, _1);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {}, _1 {}): (_1, _1)) -> (_1, _1) {
        (_1, _1)
    }
}
/// `add_carry(1, q:0) ==> succ(q):0`
impl<RHS: Pos, Rec> Fn<(_1, (RHS, _0))> for AddCarry where
    Succ: Fn(RHS) -> Rec
{
    type Output = (Rec, _0);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {}, (rhs, _0 {})): (_1, (RHS, _0))) -> (Rec, _0) {
        (Succ(rhs), _0)
    }
}
/// `add_carry(1, q:1) ==> succ(q):1`
impl<RHS: Pos, Rec> Fn<(_1, (RHS, _1))> for AddCarry where
    Succ: Fn(RHS) -> Rec
{
    type Output = (Rec, _1);
    #[inline]
    extern "rust-call" fn call(&self, (_1 {}, (rhs, _1 {})): (_1, (RHS, _1))) -> (Rec, _1) {
        (Succ(rhs), _1)
    }
}
/// `add_carry(p:0, 1) ==> succ(p):0`
impl<LHS: Pos, Rec> Fn<((LHS, _0), _1)> for AddCarry where
    Succ: Fn(LHS) -> Rec
{
    type Output = (Rec, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _0 {},), _1 {}): ((LHS, _0), _1)) -> (Rec, _0) {
        (Succ(lhs), _0)
    }
}
/// `add_carry(p:0, q:0) ==> add(p, q):1`
impl<LHS: Pos, RHS: Pos, Rec> Fn<((LHS, _0), (RHS, _0))> for AddCarry where
    Add: Fn(LHS, RHS) -> Rec
{
    type Output = (Rec, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _0 {}), (rhs, _0 {})): ((LHS, _0), (RHS, _0))) -> (Rec, _1) {
        (Add(lhs, rhs), _1)
    }
}
/// `add_carry(p:0, q:1) ==> add_carry(p, q):0`
impl<LHS: Pos, RHS: Pos, Rec> Fn<((LHS, _0), (RHS, _1))> for AddCarry where
    AddCarry: Fn(LHS, RHS) -> Rec
{
    type Output = (Rec, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _0 {}), (rhs, _1 {})): ((LHS, _0), (RHS, _1))) -> (Rec, _0) {
        (AddCarry(lhs, rhs), _0)
    }
}
/// `add_carry(p:1, 1) ==> succ(p):1`
impl<LHS: Pos, Rec> Fn<((LHS, _1), _1)> for AddCarry where
    Succ: Fn(LHS) -> Rec
{
    type Output = (Rec, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _1 {}), _1 {}): ((LHS, _1), _1)) -> (Rec, _1) {
        (Succ(lhs), _1)
    }
}
/// `add_carry(p:1, q:0) ==> add_carry(p, q):0`
impl<LHS: Pos, RHS: Pos, Rec> Fn<((LHS, _1), (RHS, _0,))> for AddCarry where
    AddCarry: Fn<(LHS, RHS), Output = Rec>
{
    type Output = (Rec, _0);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _1 {}), (rhs, _0 {})): ((LHS, _1), (RHS, _0))) -> (Rec, _0) {
        (AddCarry(lhs, rhs), _0)
    }
}
/// `add_carry(p:1, q:1) ==> add_carry(p, q):1`
impl<LHS: Pos, RHS: Pos, Rec> Fn<((LHS, _1), (RHS, _1,))> for AddCarry where
    AddCarry: Fn<(LHS, RHS), Output = Rec>
{
    type Output = (Rec, _1);
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _1 {}), (rhs, _1 {})): ((LHS, _1), (RHS, _1))) -> (Rec, _1) {
        (AddCarry(lhs, rhs), _1)
    }
}
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
    }
}

// Fn: CompareCont /////////////////////////////////////////////////////////////

// `compare_cont(1, 1, k) ==> k`
impl<K> Fn<(_1, _1, K)> for CompareCont {
    type Output = K;
    #[inline]
    extern "rust-call" fn call(&self, (_1 {}, _1 {}, k): (_1, _1, K)) -> K {
        k
    }
}
// `compare_cont(1, q:b, k) ==> lt`
impl<P: Pos, B: Bit, K> Fn<(_1, (P, B), K)> for CompareCont {
    type Output = order::LT;
    #[inline]
    extern "rust-call" fn call(&self, (_1 {}, (_p, _b), _k): (_1, (P, B), K)) -> order::LT {
        order::LT
    }
}
// `compare_cont(p:0, 1, k) ==> gt`
impl<LHS: Pos, K> Fn<((LHS, _0), _1, K)> for CompareCont {
    type Output = order::GT;
    #[inline]
    extern "rust-call" fn call(&self, ((_lhs, _0 {}), _1 {}, _k): ((LHS, _0), _1, K)) -> order::GT {
        order::GT
    }
}
// `compare_cont(p:0, q:0, k) ==> compare_cont(p, q, k)`
impl<LHS: Pos, RHS: Pos, K, Rec> Fn<((LHS, _0), (RHS, _0), K)> for CompareCont where
    CompareCont: Fn(LHS, RHS, K) -> Rec
{
    type Output = Rec;
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _0 {}), (rhs, _0 {}), k): ((LHS, _0), (RHS, _0), K)) -> Rec {
        CompareCont(lhs, rhs, k)
    }
}
// `compare_cont(p:0, q:1, k) ==> compare_cont(p, q, lt)`
impl<LHS: Pos, RHS: Pos, K, Rec> Fn<((LHS, _0), (RHS, _1), K)> for CompareCont where
    CompareCont: Fn(LHS, RHS, order::LT) -> Rec
{
    type Output = Rec;
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _0 {}), (rhs, _1 {}), _k): ((LHS, _0), (RHS, _1), K)) -> Rec {
        CompareCont(lhs, rhs, order::LT)
    }
}
// `compare_cont(p:1, 1, k) ==> gt`
impl<LHS: Pos, K> Fn<((LHS, _1), _1, K)> for CompareCont {
    type Output = order::GT;
    #[inline]
    extern "rust-call" fn call(&self, ((_lhs, _1 {}), _1 {}, _k): ((LHS, _1), _1, K)) -> order::GT {
        order::GT
    }
}
// `compare_cont(p:1, q:0, k) ==> compare_cont(p, q, gt)`
impl<LHS: Pos, RHS: Pos, K, Rec> Fn<((LHS, _1), (RHS, _0), K)> for CompareCont where
    CompareCont: Fn(LHS, RHS, order::GT) -> Rec
{
    type Output = Rec;
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _1 {}), (rhs, _0 {}), _): ((LHS, _1), (RHS, _0), K)) -> Rec {
        CompareCont(lhs, rhs, order::GT)
    }
}
// `compare_cont(p:1, q:1, k) ==> compare_cont(p, q, k)`
impl<LHS: Pos, RHS: Pos, K, Rec> Fn<((LHS, _1), (RHS, _1), K)> for CompareCont where
    CompareCont: Fn(LHS, RHS, K) -> Rec
{
    type Output = Rec;
    #[inline]
    extern "rust-call" fn call(&self, ((lhs, _1 {}), (rhs, _1 {}), k): ((LHS, _1), (RHS, _1), K)) -> Rec {
        CompareCont(lhs, rhs, k)
    }
}

// Fn: Compare /////////////////////////////////////////////////////////////////

impl<LHS: IsNat, RHS: IsNat, Rec> Fn<(W<LHS>, W<RHS>)> for Compare where
    Compare: Fn(LHS, RHS) -> Rec
{
    type Output = Rec;
    #[inline]
    extern "rust-call" fn call(&self, (W(lhs), W(rhs)): (W<LHS>, W<RHS>)) -> Rec {
        Compare(lhs, rhs)
    }
}

impl Fn<(_0, _0)> for Compare {
    type Output = order::Eq;
    #[inline]
    extern "rust-call" fn call(&self, (_0 {}, _0 {}): (_0, _0)) -> order::Eq {
        order::Eq
    }
}
impl<RHS: Pos> Fn<(_0, RHS)> for Compare {
    type Output = order::LT;
    #[inline]
    extern "rust-call" fn call(&self, (_0 {}, _rhs): (_0, RHS)) -> order::LT {
        order::LT
    }
}
impl<LHS: Pos> Fn<(LHS, _0)> for Compare {
    type Output = order::GT;
    #[inline]
    extern "rust-call" fn call(&self, (_lhs, _0 {}): (LHS, _0)) -> order::GT {
        order::GT
    }
}
impl<LHS: Pos, RHS: Pos, Rec> Fn<(LHS, RHS)> for Compare where
    CompareCont: Fn(LHS, RHS, order::Eq) -> Rec
{
    type Output = Rec;
    #[inline]
    extern "rust-call" fn call(&self, (lhs, rhs): (LHS, RHS)) -> Rec {
        CompareCont(lhs, rhs, order::Eq)
    }
}
