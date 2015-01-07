use ty;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Z {}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum S<N: Nat> {}

pub trait Nat {}
impl Nat for Z {}
impl<N: Nat> Nat for S<N> {}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Add {}
impl<RHS: Nat> ty::fun::Fn<Add, ( Z, RHS, )> for Add
{
    type Out = RHS;
}
impl<LHS: Nat, RHS: Nat> ty::fun::Fn<Add, ( S<LHS>, RHS, )> for Add where
    Add: ty::fun::Fn<Add, ( LHS, RHS ,)>,
{
    type Out = S<<Add as ty::fun::Fn<Add, ( LHS, RHS, )>>::Out>;
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Mul {}
impl<RHS: Nat> ty::fun::Fn<Mul, ( Z, RHS, )> for Mul
{
    type Out = Z;
}
impl<LHS: Nat, RHS: Nat, Rec: Nat> ty::fun::Fn<Mul, ( S<LHS>, RHS, )> for Mul where
    Mul: ty::fun::Fn<Mul, ( LHS, RHS, ), Out = Rec>,
    Add: ty::fun::Fn<Add, ( RHS, Rec, )>,
{
    type Out = <Add as ty::fun::Fn<Add, ( RHS, Rec, )>>::Out;
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Exp {}
impl<RHS: Nat> ty::fun::Fn<Exp, ( Z, RHS, )> for Exp
{
    type Out = S<Z>;
}
impl<LHS: Nat, RHS: Nat, Rec: Nat> ty::fun::Fn<Exp, ( S<LHS>, RHS, )> for Exp where
    Exp: ty::fun::Fn<Exp, ( LHS, RHS, ), Out = Rec>,
    Mul: ty::fun::Fn<Mul, ( RHS, Rec, )>,
{
    type Out = <Mul as ty::fun::Fn<Mul, ( RHS, Rec, )>>::Out;
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Fac {}
impl ty::fun::Fn<Fac, ( Z, )> for Fac
{
    type Out = S<Z>;
}
impl<LHS: Nat, Rec: Nat> ty::fun::Fn<Fac, ( S<LHS>, )> for Fac where
    Fac: ty::fun::Fn<Fac, ( LHS, ), Out = Rec>,
    Mul: ty::fun::Fn<Mul, ( S<LHS>, Rec, )>,
{
    type Out = <Mul as ty::fun::Fn<Mul, ( S<LHS>, Rec, )>>::Out;
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
    use ty;
    use super::{
        Add,
        Mul,
        Exp,
        Fac,
        N02,
        N03,
        N04,
        N05,
        N06,
        N08,
        N24,
    };

    #[test]
    fn add() { let _: ty::fun::Val< N05 > = ty::fun::Val::val::<Add, ( N03, N02, )>(); }

    #[test]
    fn mul() { let _: ty::fun::Val< N06 > = ty::fun::Val::val::<Mul, ( N03, N02, )>(); }

    #[test]
    fn exp() { let _: ty::fun::Val< N08 > = ty::fun::Val::val::<Exp, ( N03, N02, )>(); }

    #[test]
    fn fac() { let _: ty::fun::Val< N24 > = ty::fun::Val::val::<Fac, ( N04, ) >(); }
}