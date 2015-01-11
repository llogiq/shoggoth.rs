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
pub enum Z {}

/// Type-level natural number successor of n
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum S<N: Nat> {}

/// Predicate classifying type-level natural numbers
pub trait Nat {}
impl Nat for Z {}
impl<N: Nat> Nat for S<N> {}

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
impl<LHS: Nat> fun::Fn<(S<LHS>,)> for Pred
{
    type O = LHS;
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
impl<RHS: Nat> fun::Fn<(Z, RHS,)> for Add
{
    type O = RHS;
}
impl<LHS: Nat, RHS: Nat> fun::Fn<(S<LHS>, RHS,)> for Add where
    Add: fun::Fn<(LHS, RHS,)>,
{
    type O = S<fun::Ap<Add, (LHS, RHS,)>>;
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
impl<LHS: Nat> fun::Fn<(LHS, Z,)> for Sub
{
    type O = LHS;
}
impl<LHS: Nat, RHS: Nat, Rec: Nat> fun::Fn<(S<LHS>, S<RHS>,)> for Sub where
    Sub: fun::Fn<(LHS, RHS,), O = Rec>,
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
impl<RHS: Nat> fun::Fn<(Z, RHS,)> for Mul
{
    type O = Z;
}
impl<LHS: Nat, RHS: Nat, Rec: Nat> fun::Fn<(S<LHS>, RHS,)> for Mul where
    Mul: fun::Fn<(LHS, RHS,), O = Rec>,
    Add: fun::Fn<(RHS, Rec,)>,
{
    type O = fun::Ap<Add, (RHS, Rec,)>;
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
impl<RHS: Nat> fun::Fn<(Z, RHS,)> for Exp
{
    type O = S<Z>;
}
impl<LHS: Nat, RHS: Nat, Rec: Nat> fun::Fn<(S<LHS>, RHS,)> for Exp where
    Exp: fun::Fn<(LHS, RHS,), O = Rec>,
    Mul: fun::Fn<(RHS, Rec,)>,
{
    type O = fun::Ap<Mul, (RHS, Rec,)>;
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
impl fun::Fn<(Z,)> for Fac
{
    type O = S<Z>;
}
impl<LHS: Nat, Rec: Nat> fun::Fn<(S<LHS>,)> for Fac where
    Fac: fun::Fn<(LHS,), O = Rec>,
    Mul: fun::Fn<(S<LHS>, Rec,)>,
{
    type O = fun::Ap<Mul, (S<LHS>, Rec,)>;
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
impl<RHS: Nat> fun::Fn<(Z, RHS,)> for LT {
    type O = True;
}
impl<LHS: Nat> fun::Fn<(S<LHS>, Z,)> for LT {
    type O = False;
}
impl<LHS: Nat, RHS: Nat> fun::Fn<(S<LHS>, S<RHS>,)> for LT where
    LT: fun::Fn<(LHS, RHS,)>,
{
    type O = fun::Ap<LT, (LHS, RHS,)>;
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
impl fun::Fn<(Z, Z,)> for LTEq {
    type O = True;
}
impl<LHS: Nat> fun::Fn<(S<LHS>, Z,)> for LTEq {
    type O = False;
}
impl<RHS: Nat> fun::Fn<(Z, S<RHS>,)> for LTEq {
    type O = True;
}
impl<LHS: Nat, RHS: Nat> fun::Fn<(S<LHS>, S<RHS>,)> for LTEq where
    LTEq: fun::Fn<(LHS, RHS,)>,
{
    type O = fun::Ap<LTEq, (LHS, RHS,)>;
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
    LTEq: fun::Fn<(LHS, RHS), O = Rec>,
    If: fun::Fn<(Rec, LHS, RHS,)>,
{
    type O = fun::Ap<If, (Rec, LHS, RHS,)>;
}

pub type N00 = Z;
pub type N01 = S<N00>;
pub type N02 = S<N01>;
pub type N03 = S<N02>;
pub type N04 = S<N03>;
pub type N05 = S<N04>;
pub type N06 = S<N05>;
pub type N07 = S<N06>;
pub type N08 = S<N07>;
pub type N09 = S<N08>;
pub type N10 = S<N09>;
pub type N11 = S<N10>;
pub type N12 = S<N11>;
pub type N13 = S<N12>;
pub type N14 = S<N13>;
pub type N15 = S<N14>;
pub type N16 = S<N15>;
pub type N17 = S<N16>;
pub type N18 = S<N17>;
pub type N19 = S<N18>;
pub type N20 = S<N19>;
pub type N21 = S<N20>;
pub type N22 = S<N21>;
pub type N23 = S<N22>;
pub type N24 = S<N23>;
pub type N25 = S<N24>;
pub type N26 = S<N25>;
pub type N27 = S<N26>;
pub type N28 = S<N27>;
pub type N29 = S<N28>;
pub type N30 = S<N29>;
pub type N31 = S<N30>;
pub type N32 = S<N31>;
pub type N33 = S<N32>;
pub type N34 = S<N33>;
pub type N35 = S<N34>;
pub type N36 = S<N35>;
pub type N37 = S<N36>;
pub type N38 = S<N37>;
pub type N39 = S<N38>;
pub type N40 = S<N39>;
pub type N41 = S<N40>;
pub type N42 = S<N41>;
pub type N43 = S<N42>;
pub type N44 = S<N43>;
pub type N45 = S<N44>;
pub type N46 = S<N45>;
pub type N47 = S<N46>;
pub type N48 = S<N47>;
pub type N49 = S<N48>;
pub type N50 = S<N49>;
pub type N51 = S<N50>;
pub type N52 = S<N51>;
pub type N53 = S<N52>;
pub type N54 = S<N53>;
pub type N55 = S<N54>;
pub type N56 = S<N55>;
pub type N57 = S<N56>;
pub type N58 = S<N57>;
pub type N59 = S<N58>;
pub type N60 = S<N59>;
pub type N61 = S<N60>;
pub type N62 = S<N61>;
pub type N63 = S<N62>;
pub type N64 = S<N63>;
pub type N65 = S<N64>;
pub type N66 = S<N65>;
pub type N67 = S<N66>;
pub type N68 = S<N67>;
pub type N69 = S<N68>;
pub type N70 = S<N69>;
pub type N71 = S<N70>;
pub type N72 = S<N71>;
pub type N73 = S<N72>;
pub type N74 = S<N73>;
pub type N75 = S<N74>;
pub type N76 = S<N75>;
pub type N77 = S<N76>;
pub type N78 = S<N77>;
pub type N79 = S<N78>;
pub type N80 = S<N79>;
pub type N81 = S<N80>;
pub type N82 = S<N81>;
pub type N83 = S<N82>;
pub type N84 = S<N83>;
pub type N85 = S<N84>;
pub type N86 = S<N85>;
pub type N87 = S<N86>;
pub type N88 = S<N87>;
pub type N89 = S<N88>;
pub type N90 = S<N89>;
pub type N91 = S<N90>;
pub type N92 = S<N91>;
pub type N93 = S<N92>;
pub type N94 = S<N93>;
pub type N95 = S<N94>;
pub type N96 = S<N95>;
pub type N97 = S<N96>;
pub type N98 = S<N97>;
pub type N99 = S<N98>;

#[cfg(test)]
mod tests {
    use ty::bool::{
        False,
        True,
    };
    use ty::fun::{
        Val,
        val,
    };
    use super::{
        Add,
        Exp,
        Fac,
        LT,
        LTEq,
        Min,
        Mul,
        N01,
        N02,
        N03,
        N04,
        N05,
        N06,
        N08,
        N24,
        Pred,
        Sub,
    };

    #[test]
    fn pred() { let _: Val<N01> = val::<Pred, (N02,)>(); }

    #[test]
    fn add() { let _: Val<N05> = val::<Add, (N03, N02,)>(); }

    #[test]
    fn sub() { let _: Val<N03> = val::<Sub, (N05, N02,)>(); }

    #[test]
    fn mul() { let _: Val<N06> = val::<Mul, (N03, N02,)>(); }

    #[test]
    fn exp() { let _: Val<N08> = val::<Exp, (N03, N02,)>(); }

    #[test]
    fn fac() { let _: Val<N24> = val::<Fac, (N04,) >(); }

    #[test]
    fn lt_false() { let _: Val<False> = val::<LT, (N04, N02,)>(); }

    #[test]
    fn lt_true() { let _: Val<True> = val::<LT, (N02, N04,)>(); }

    #[test]
    fn lteq_false() { let _: Val<False> = val::<LTEq, (N04, N02,)>(); }

    #[test]
    fn lteq_true_00() { let _: Val<True> = val::<LTEq, (N02, N02,)>(); }

    #[test]
    fn lteq_true_01() { let _: Val<True> = val::<LTEq, (N02, N04,)>(); }

    #[test]
    fn min_lhs() { let _: Val<N02> = val::<Min, (N02, N04,)>(); }

    #[test]
    fn min_rhs() { let _: Val<N02> = val::<Min, (N04, N02,)>(); }
}
