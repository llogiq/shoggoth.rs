use ty::{
    FnTm,
    Sig,
    Tm,
    Ty,
    bool,
};

/// Type-level nat
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Nat {}
impl Ty for Nat {}

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
impl Tm<Nat> for Zero {}

/// Type-level natural number successor of n
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Succ<N: Tm<Nat>> {}
impl<N: Tm<Nat>> Tm<Nat> for Succ<N> {}

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
impl Sig for Pred { type Dom = (Nat,); type Cod = Nat; }
impl<N: Tm<Nat>> FnTm<Pred> for (Succ<N>,)
{
    type O = N;
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
impl Sig for Add { type Dom = (Nat, Nat,); type Cod = Nat; }
impl<N1: Tm<Nat>> FnTm<Add> for (Zero, N1,)
{
    type O = N1;
}
impl<N0: Tm<Nat>, N1: Tm<Nat>, Rec: Tm<Nat>> FnTm<Add> for (Succ<N0>, N1,) where
    (N0, N1,): FnTm<Add, O = Rec>,
{
    type O = Succ<Rec>;
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
impl Sig for Sub { type Dom = (Nat, Nat,); type Cod = Nat; }
impl<N0: Tm<Nat>> FnTm<Sub> for (N0, Zero,)
{
    type O = N0;
}
impl<N0: Tm<Nat>, N1: Tm<Nat>, Rec: Tm<Nat>> FnTm<Sub> for (Succ<N0>, Succ<N1>,) where
    (N0, N1,): FnTm<Sub, O = Rec>,
{
    type O = Rec;
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
impl Sig for Mul { type Dom = (Nat, Nat,); type Cod = Nat; }
impl<N1: Tm<Nat>> FnTm<Mul> for (Zero, N1,)
{
    type O = Zero;
}
impl<N0: Tm<Nat>, N1: Tm<Nat>, Rec0: Tm<Nat>, Rec1: Tm<Nat>> FnTm<Mul> for (Succ<N0>, N1,) where
    (N0, N1,): FnTm<Mul, O = Rec0>,
    (N1, Rec0,): FnTm<Add, O = Rec1>,
{
    type O = Rec1;
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
impl Sig for Exp { type Dom = (Nat, Nat,); type Cod = Nat; }
impl<N1: Tm<Nat>> FnTm<Exp> for (Zero, N1,)
{
    type O = Succ<Zero>;
}
impl<N0: Tm<Nat>, N1: Tm<Nat>, Rec0: Tm<Nat>, Rec1: Tm<Nat>> FnTm<Exp> for (Succ<N0>, N1,) where
    (N0, N1,): FnTm<Exp, O = Rec0>,
    (N1, Rec0,): FnTm<Mul, O = Rec1>,
{
    type O = Rec1;
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
impl Sig for Fac { type Dom = (Nat,); type Cod = Nat; }
impl FnTm<Fac> for (Zero,)
{
    type O = Succ<Zero>;
}
impl<N0: Tm<Nat>, Rec0: Tm<Nat>, Rec1: Tm<Nat>> FnTm<Fac> for (Succ<N0>,) where
    (N0,): FnTm<Fac, O = Rec0>,
    (Succ<N0>, Rec0,): FnTm<Mul, O = Rec1>,
{
    type O = Rec1;
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
impl Sig for LT { type Dom = (Nat, Nat,); type Cod = bool::Bool; }
impl<N1: Tm<Nat>> FnTm<LT> for (Zero, N1,) {
    type O = bool::True;
}
impl<N0: Tm<Nat>> FnTm<LT> for (Succ<N0>, Zero,) {
    type O = bool::False;
}
impl<N0: Tm<Nat>, N1: Tm<Nat>, Rec: Tm<bool::Bool>> FnTm<LT> for (Succ<N0>, Succ<N1>,) where
    (N0, N1,): FnTm<LT, O = Rec>,
{
    type O = Rec;
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
impl Sig for LTEq { type Dom = (Nat, Nat,); type Cod = bool::Bool; }
impl FnTm<LTEq> for (Zero, Zero,) {
    type O = bool::True;
}
impl<N0: Tm<Nat>> FnTm<LTEq> for (Succ<N0>, Zero,) {
    type O = bool::False;
}
impl<N1: Tm<Nat>> FnTm<LTEq> for (Zero, Succ<N1>,) {
    type O = bool::True;
}
impl<N0: Tm<Nat>, N1: Tm<Nat>, Rec: Tm<bool::Bool>> FnTm<LTEq> for (Succ<N0>, Succ<N1>,) where
    (N0, N1,): FnTm<LTEq, O = Rec>,
{
    type O = Rec;
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
impl Sig for Min { type Dom = (Nat, Nat,); type Cod = Nat; }
impl<N0: Tm<Nat>, N1: Tm<Nat>, Rec0: Tm<bool::Bool>, Rec1: Tm<Nat>> FnTm<Min> for (N0, N1,) where
    (N0, N1): Tm<(Nat, Nat)>, // FIXME: coherence failed to report ambiguity
    (N0, N1): FnTm<LTEq, O = Rec0>,
    (Rec0, N0, N1,): FnTm<bool::If<Nat>, O = Rec1>,
{
    type O = Rec1;
}

#[cfg(test)]
mod tests {
    use super::*;
    use ty::literal::*;
    use ty::wit::*;

    #[test]
    fn pred() { let _: Wit<_1n> = wit::<Pred, (_2n,)>(); }

    #[test]
    fn add() { let _: Wit<_5n> = wit::<Add, (_3n, _2n,)>(); }

    #[test]
    fn sub() { let _: Wit<_3n> = wit::<Sub, (_5n, _2n,)>(); }

    #[test]
    fn mul() { let _: Wit<_6n> = wit::<Mul, (_3n, _2n,)>(); }

    #[test]
    fn exp() { let _: Wit<_8n> = wit::<Exp, (_3n, _2n,)>(); }

    #[test]
    fn fac() { let _: Wit<_24n> = wit::<Fac, (_4n,) >(); }

    #[test]
    fn lt_false() { let _: Wit<FF> = wit::<LT, (_4n, _2n,)>(); }

    #[test]
    fn lt_true() { let _: Wit<TT> = wit::<LT, (_2n, _4n,)>(); }

    #[test]
    fn lteq_false() { let _: Wit<FF> = wit::<LTEq, (_4n, _2n,)>(); }

    #[test]
    fn lteq_true_00() { let _: Wit<TT> = wit::<LTEq, (_2n, _2n,)>(); }

    #[test]
    fn lteq_true_01() { let _: Wit<TT> = wit::<LTEq, (_2n, _4n,)>(); }

    #[test]
    fn min_lhs() { let _: Wit<_2n> = wit::<Min, (_2n, _4n,)>(); }

    #[test]
    fn min_rhs() { let _: Wit<_2n> = wit::<Min, (_4n, _2n,)>(); }
}
