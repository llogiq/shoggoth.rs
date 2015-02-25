use bit::{
    _0,
    _1,
};
use nat::{
    Pos,
};
use nat::ops::*;

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
