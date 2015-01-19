use hlist::{
    HC,
    HN,
};
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
impl Sig for Pred { type Dom = Nat; type Cod = Nat; }
impl<N: Tm<Nat>> FnTm<Pred> for Succ<N>
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
impl Sig for Add { type Dom = HC<Nat, HC<Nat, HN>>; type Cod = Nat; }
impl<N1: Tm<Nat>> FnTm<Add> for HC<Zero, HC<N1, HN>>
{
    type O = N1;
}
impl<N0: Tm<Nat>, N1: Tm<Nat>, Rec: Tm<Nat>> FnTm<Add> for HC<Succ<N0>, HC<N1, HN>> where
    HC<N0, HC<N1, HN>>: FnTm<Add, O = Rec>,
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
impl Sig for Sub { type Dom = HC<Nat, HC<Nat, HN>>; type Cod = Nat; }
impl<N0: Tm<Nat>> FnTm<Sub> for HC<N0, HC<Zero, HN>>
{
    type O = N0;
}
impl<N0: Tm<Nat>, N1: Tm<Nat>, Rec: Tm<Nat>> FnTm<Sub> for HC<Succ<N0>, HC<Succ<N1>, HN>> where
    HC<N0, HC<N1, HN>>: FnTm<Sub, O = Rec>,
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
impl Sig for Mul { type Dom = HC<Nat, HC<Nat, HN>>; type Cod = Nat; }
impl<N1: Tm<Nat>> FnTm<Mul> for HC<Zero, HC<N1, HN>>
{
    type O = Zero;
}
impl<N0: Tm<Nat>, N1: Tm<Nat>, Rec0: Tm<Nat>, Rec1: Tm<Nat>> FnTm<Mul> for HC<Succ<N0>, HC<N1, HN>> where
    HC<N0, HC<N1, HN>>: FnTm<Mul, O = Rec0>,
    HC<N1, HC<Rec0, HN>>: FnTm<Add, O = Rec1>,
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
impl Sig for Exp { type Dom = HC<Nat, HC<Nat, HN>>; type Cod = Nat; }
impl<N1: Tm<Nat>> FnTm<Exp> for HC<Zero, HC<N1, HN>>
{
    type O = Succ<Zero>;
}
impl<N0: Tm<Nat>, N1: Tm<Nat>, Rec0: Tm<Nat>, Rec1: Tm<Nat>> FnTm<Exp> for HC<Succ<N0>, HC<N1, HN>> where
    HC<N0, HC<N1, HN>>: FnTm<Exp, O = Rec0>,
    HC<N1, HC<Rec0, HN>>: FnTm<Mul, O = Rec1>,
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
impl Sig for Fac { type Dom = Nat; type Cod = Nat; }
impl FnTm<Fac> for Zero
{
    type O = Succ<Zero>;
}
impl<N0: Tm<Nat>, Rec0: Tm<Nat>, Rec1: Tm<Nat>> FnTm<Fac> for Succ<N0> where
    N0: FnTm<Fac, O = Rec0>,
    HC<Succ<N0>, HC<Rec0, HN>>: FnTm<Mul, O = Rec1>,
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
impl Sig for LT { type Dom = HC<Nat, HC<Nat, HN>>; type Cod = bool::Bool; }
impl<N1: Tm<Nat>> FnTm<LT> for HC<Zero, HC<N1, HN>> {
    type O = bool::True;
}
impl<N0: Tm<Nat>> FnTm<LT> for HC<Succ<N0>, HC<Zero, HN>> {
    type O = bool::False;
}
impl<N0: Tm<Nat>, N1: Tm<Nat>, Rec: Tm<bool::Bool>> FnTm<LT> for HC<Succ<N0>, HC<Succ<N1>, HN>> where
    HC<N0, HC<N1, HN>>: FnTm<LT, O = Rec>,
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
impl Sig for LTEq { type Dom = HC<Nat, HC<Nat, HN>>; type Cod = bool::Bool; }
impl FnTm<LTEq> for HC<Zero, HC<Zero, HN>> {
    type O = bool::True;
}
impl<N0: Tm<Nat>> FnTm<LTEq> for HC<Succ<N0>, HC<Zero, HN>> {
    type O = bool::False;
}
impl<N1: Tm<Nat>> FnTm<LTEq> for HC<Zero, HC<Succ<N1>, HN>> {
    type O = bool::True;
}
impl<N0: Tm<Nat>, N1: Tm<Nat>, Rec: Tm<bool::Bool>> FnTm<LTEq> for HC<Succ<N0>, HC<Succ<N1>, HN>> where
    HC<N0, HC<N1, HN>>: FnTm<LTEq, O = Rec>,
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
impl Sig for Min { type Dom = HC<Nat, HC<Nat, HN>>; type Cod = Nat; }
impl<N0: Tm<Nat>, N1: Tm<Nat>, Rec0: Tm<bool::Bool>, Rec1: Tm<Nat>> FnTm<Min> for HC<N0, HC<N1, HN>> where
    HC<N0, HC<N1, HN>>: Tm<HC<Nat, HC<Nat, HN>>>, // FIXME: coherence failed to report ambiguity
    HC<N0, HC<N1, HN>>: FnTm<LTEq, O = Rec0>,
    HC<Rec0, HC<N0, HC<N1, HN>>>: FnTm<bool::If<Nat>, O = Rec1>,
{
    type O = Rec1;
}

#[cfg(test)]
mod tests {
    use super::*;
    use ty::literal::*;
    use ty::wit::*;

    #[test]
    fn pred() { let _: Wit<_1n> = wit::<Pred, _2n>(); }

    #[test]
    fn add() { let _: Wit<_5n> = wit::<Add, HC<_3n, HC<_2n, HN>>>(); }

    #[test]
    fn sub() { let _: Wit<_3n> = wit::<Sub, HC<_5n, HC<_2n, HN>>>(); }

    #[test]
    fn mul() { let _: Wit<_6n> = wit::<Mul, HC<_3n, HC<_2n, HN>>>(); }

    #[test]
    fn exp() { let _: Wit<_8n> = wit::<Exp, HC<_3n, HC<_2n, HN>>>(); }

    #[test]
    fn fac() { let _: Wit<_24n> = wit::<Fac, _4n>(); }

    #[test]
    fn lt_false() { let _: Wit<FF> = wit::<LT, HC<_4n, HC<_2n, HN>>>(); }

    #[test]
    fn lt_true() { let _: Wit<TT> = wit::<LT, HC<_2n, HC<_4n, HN>>>(); }

    #[test]
    fn lteq_false() { let _: Wit<FF> = wit::<LTEq, HC<_4n, HC<_2n, HN>>>(); }

    #[test]
    fn lteq_true_00() { let _: Wit<TT> = wit::<LTEq, HC<_2n, HC<_2n, HN>>>(); }

    #[test]
    fn lteq_true_01() { let _: Wit<TT> = wit::<LTEq, HC<_2n, HC<_4n, HN>>>(); }

    #[test]
    fn min_lhs() { let _: Wit<_2n> = wit::<Min, HC<_2n, HC<_4n, HN>>>(); }

    #[test]
    fn min_rhs() { let _: Wit<_2n> = wit::<Min, HC<_4n, HC<_2n, HN>>>(); }
}
