use ty::bool::{
    Bool,
    False,
    If,
    True,
};
use ty::fun;

/// Type-level natural number zero
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Zero {}

/// Type-level natural number successor of n
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Succ<N: Nat> {}

/// Predicate classifying type-level natural numbers
pub trait Nat {}
impl Nat for Zero {}
impl<N: Nat> Nat for Succ<N> {}

/// Type-level function for nat predecessor
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Pred {}
impl<LHS: Nat> fun::Fn<(Succ<LHS>,)> for Pred
{
    type Out = LHS;
}

/// Type-level function for nat addition
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Add {}
impl<RHS: Nat> fun::Fn<(Zero, RHS,)> for Add
{
    type Out = RHS;
}
impl<LHS: Nat, RHS: Nat> fun::Fn<(Succ<LHS>, RHS,)> for Add where
    Add: fun::Fn<(LHS, RHS,)>,
{
    type Out = Succ<fun::Ap<Add, (LHS, RHS,)>>;
}

/// Type-level function for nat subtraction
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Sub {}
impl<LHS: Nat> fun::Fn<(LHS, Zero,)> for Sub
{
    type Out = LHS;
}
impl<LHS: Nat, RHS: Nat, Rec: Nat> fun::Fn<(Succ<LHS>, Succ<RHS>,)> for Sub where
    Sub: fun::Fn<(LHS, RHS,), Out = Rec>,
{
    type Out = Rec;
}

/// Type-level function for nat multiplication
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Mul {}
impl<RHS: Nat> fun::Fn<(Zero, RHS,)> for Mul
{
    type Out = Zero;
}
impl<LHS: Nat, RHS: Nat, Rec: Nat> fun::Fn<(Succ<LHS>, RHS,)> for Mul where
    Mul: fun::Fn<(LHS, RHS,), Out = Rec>,
    Add: fun::Fn<(RHS, Rec,)>,
{
    type Out = fun::Ap<Add, (RHS, Rec,)>;
}

/// Type-level function for nat exponentiation
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Exp {}
impl<RHS: Nat> fun::Fn<(Zero, RHS,)> for Exp
{
    type Out = Succ<Zero>;
}
impl<LHS: Nat, RHS: Nat, Rec: Nat> fun::Fn<(Succ<LHS>, RHS,)> for Exp where
    Exp: fun::Fn<(LHS, RHS,), Out = Rec>,
    Mul: fun::Fn<(RHS, Rec,)>,
{
    type Out = fun::Ap<Mul, (RHS, Rec,)>;
}

/// Type-level function for nat factorial
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Fac {}
impl fun::Fn<(Zero,)> for Fac
{
    type Out = Succ<Zero>;
}
impl<LHS: Nat, Rec: Nat> fun::Fn<(Succ<LHS>,)> for Fac where
    Fac: fun::Fn<(LHS,), Out = Rec>,
    Mul: fun::Fn<(Succ<LHS>, Rec,)>,
{
    type Out = fun::Ap<Mul, (Succ<LHS>, Rec,)>;
}

/// Type-level function for nat less-than
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum LT {}
impl<RHS: Nat> fun::Fn<(Zero, RHS,)> for LT {
    type Out = True;
}
impl<LHS: Nat> fun::Fn<(Succ<LHS>, Zero,)> for LT {
    type Out = False;
}
impl<LHS: Nat, RHS: Nat> fun::Fn<(Succ<LHS>, Succ<RHS>,)> for LT where
    LT: fun::Fn<(LHS, RHS,)>,
{
    type Out = fun::Ap<LT, (LHS, RHS,)>;
}

/// Type-level function for nat less-than-or-eq
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum LTEq {}
impl fun::Fn<(Zero, Zero,)> for LTEq {
    type Out = True;
}
impl<LHS: Nat> fun::Fn<(Succ<LHS>, Zero,)> for LTEq {
    type Out = False;
}
impl<RHS: Nat> fun::Fn<(Zero, Succ<RHS>,)> for LTEq {
    type Out = True;
}
impl<LHS: Nat, RHS: Nat> fun::Fn<(Succ<LHS>, Succ<RHS>,)> for LTEq where
    LTEq: fun::Fn<(LHS, RHS,)>,
{
    type Out = fun::Ap<LTEq, (LHS, RHS,)>;
}

/// Type-level function for nat min
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Min {}
impl<LHS: Nat, RHS: Nat, Rec: Bool> fun::Fn<(LHS, RHS,)> for Min where
    LTEq: fun::Fn<(LHS, RHS), Out = Rec>,
    If: fun::Fn<(Rec, LHS, RHS,)>,
{
    type Out = fun::Ap<If, (Rec, LHS, RHS,)>;
}

#[cfg(test)]
mod tests {
    use ty::bool::*;
    use ty::literal::*;
    use ty::nat::*;
    use ty::val::*;

    #[test]
    fn pred() { let _: Val<_1n> = val::<Pred, (_2n,)>(); }

    #[test]
    fn add() { let _: Val<_5n> = val::<Add, (_3n, _2n,)>(); }

    #[test]
    fn sub() { let _: Val<_3n> = val::<Sub, (_5n, _2n,)>(); }

    #[test]
    fn mul() { let _: Val<_6n> = val::<Mul, (_3n, _2n,)>(); }

    #[test]
    fn exp() { let _: Val<_8n> = val::<Exp, (_3n, _2n,)>(); }

    #[test]
    fn fac() { let _: Val<_24n> = val::<Fac, (_4n,) >(); }

    #[test]
    fn lt_false() { let _: Val<False> = val::<LT, (_4n, _2n,)>(); }

    #[test]
    fn lt_true() { let _: Val<True> = val::<LT, (_2n, _4n,)>(); }

    #[test]
    fn lteq_false() { let _: Val<False> = val::<LTEq, (_4n, _2n,)>(); }

    #[test]
    fn lteq_true_00() { let _: Val<True> = val::<LTEq, (_2n, _2n,)>(); }

    #[test]
    fn lteq_true_01() { let _: Val<True> = val::<LTEq, (_2n, _4n,)>(); }

    #[test]
    fn min_lhs() { let _: Val<_2n> = val::<Min, (_2n, _4n,)>(); }

    #[test]
    fn min_rhs() { let _: Val<_2n> = val::<Min, (_4n, _2n,)>(); }
}
