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

pub type  _0 = Z;
pub type  _1 = S< _0>;
pub type  _2 = S< _1>;
pub type  _3 = S< _2>;
pub type  _4 = S< _3>;
pub type  _5 = S< _4>;
pub type  _6 = S< _5>;
pub type  _7 = S< _6>;
pub type  _8 = S< _7>;
pub type  _9 = S< _8>;
pub type _10 = S< _9>;
pub type _11 = S<_10>;
pub type _12 = S<_11>;
pub type _13 = S<_12>;
pub type _14 = S<_13>;
pub type _15 = S<_14>;
pub type _16 = S<_15>;
pub type _17 = S<_16>;
pub type _18 = S<_17>;
pub type _19 = S<_18>;
pub type _20 = S<_19>;
pub type _21 = S<_20>;
pub type _22 = S<_21>;
pub type _23 = S<_22>;
pub type _24 = S<_23>;
pub type _25 = S<_24>;
pub type _26 = S<_25>;
pub type _27 = S<_26>;
pub type _28 = S<_27>;
pub type _29 = S<_28>;
pub type _30 = S<_29>;
pub type _31 = S<_30>;
pub type _32 = S<_31>;
pub type _33 = S<_32>;
pub type _34 = S<_33>;
pub type _35 = S<_34>;
pub type _36 = S<_35>;
pub type _37 = S<_36>;
pub type _38 = S<_37>;
pub type _39 = S<_38>;
pub type _40 = S<_39>;
pub type _41 = S<_40>;
pub type _42 = S<_41>;
pub type _43 = S<_42>;
pub type _44 = S<_43>;
pub type _45 = S<_44>;
pub type _46 = S<_45>;
pub type _47 = S<_46>;
pub type _48 = S<_47>;
pub type _49 = S<_48>;
pub type _50 = S<_49>;
pub type _51 = S<_50>;
pub type _52 = S<_51>;
pub type _53 = S<_52>;
pub type _54 = S<_53>;
pub type _55 = S<_54>;
pub type _56 = S<_55>;
pub type _57 = S<_56>;
pub type _58 = S<_57>;
pub type _59 = S<_58>;
pub type _60 = S<_59>;
pub type _61 = S<_60>;
pub type _62 = S<_61>;
pub type _63 = S<_62>;
pub type _64 = S<_63>;
pub type _65 = S<_64>;
pub type _66 = S<_65>;
pub type _67 = S<_66>;
pub type _68 = S<_67>;
pub type _69 = S<_68>;
pub type _70 = S<_69>;
pub type _71 = S<_70>;
pub type _72 = S<_71>;
pub type _73 = S<_72>;
pub type _74 = S<_73>;
pub type _75 = S<_74>;
pub type _76 = S<_75>;
pub type _77 = S<_76>;
pub type _78 = S<_77>;
pub type _79 = S<_78>;
pub type _80 = S<_79>;
pub type _81 = S<_80>;
pub type _82 = S<_81>;
pub type _83 = S<_82>;
pub type _84 = S<_83>;
pub type _85 = S<_84>;
pub type _86 = S<_85>;
pub type _87 = S<_86>;
pub type _88 = S<_87>;
pub type _89 = S<_88>;
pub type _90 = S<_89>;
pub type _91 = S<_90>;
pub type _92 = S<_91>;
pub type _93 = S<_92>;
pub type _94 = S<_93>;
pub type _95 = S<_94>;
pub type _96 = S<_95>;
pub type _97 = S<_96>;
pub type _98 = S<_97>;
pub type _99 = S<_98>;

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
        _1,
        _2,
        _3,
        _4,
        _5,
        _6,
        _8,
        _24,
        Add,
        Exp,
        Fac,
        LT,
        LTEq,
        Min,
        Mul,
        Pred,
        Sub,
    };

    #[test]
    fn pred() { let _: Val<_1> = val::<Pred, (_2,)>(); }

    #[test]
    fn add() { let _: Val<_5> = val::<Add, (_3, _2,)>(); }

    #[test]
    fn sub() { let _: Val<_3> = val::<Sub, (_5, _2,)>(); }

    #[test]
    fn mul() { let _: Val<_6> = val::<Mul, (_3, _2,)>(); }

    #[test]
    fn exp() { let _: Val<_8> = val::<Exp, (_3, _2,)>(); }

    #[test]
    fn fac() { let _: Val<_24> = val::<Fac, (_4,) >(); }

    #[test]
    fn lt_false() { let _: Val<False> = val::<LT, (_4, _2,)>(); }

    #[test]
    fn lt_true() { let _: Val<True> = val::<LT, (_2, _4,)>(); }

    #[test]
    fn lteq_false() { let _: Val<False> = val::<LTEq, (_4, _2,)>(); }

    #[test]
    fn lteq_true_00() { let _: Val<True> = val::<LTEq, (_2, _2,)>(); }

    #[test]
    fn lteq_true_01() { let _: Val<True> = val::<LTEq, (_2, _4,)>(); }

    #[test]
    fn min_lhs() { let _: Val<_2> = val::<Min, (_2, _4,)>(); }

    #[test]
    fn min_rhs() { let _: Val<_2> = val::<Min, (_4, _2,)>(); }
}
