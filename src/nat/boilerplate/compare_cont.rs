use bit::{
    _0,
    _1,
    Bit,
};
use nat::{
    Pos,
};
use nat::ops::*;
use order;

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
