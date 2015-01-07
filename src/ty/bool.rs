use super::fun as ty;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum False {}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum True {}

pub trait Bool {}
impl Bool for False {}
impl Bool for True  {}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Not {}
impl ty::Fn<Not, False> for Not { type Out =  True; }
impl ty::Fn<Not,  True> for Not { type Out = False; }

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum And {}
impl<RHS: Bool> ty::Fn<And, ( False, RHS )> for And { type Out = False; }
impl<RHS: Bool> ty::Fn<And, (  True, RHS )> for And { type Out =   RHS; }

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Or {}
impl<RHS: Bool> ty::Fn<Or, ( False, RHS )> for Or { type Out =  RHS; }
impl<RHS: Bool> ty::Fn<Or, (  True, RHS )> for Or { type Out = True; }

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum If {}
impl<LHS, RHS> ty::Fn<If, (  True, LHS, RHS )> for If { type Out = LHS; }
impl<LHS, RHS> ty::Fn<If, ( False, LHS, RHS )> for If { type Out = RHS; }

#[cfg(test)]
mod tests {
    use ty::fun as ty;
    use super::{
        And,
        Bool,
        False,
        If,
        Not,
        Or,
        True,
    };

    #[test]
    fn not_false() { let _: ty::Val<  True > = ty::Val::val::<Not, False>(); }

    #[test]
    fn not_true () { let _: ty::Val< False > = ty::Val::val::<Not,  True>(); }

    #[test]
    fn and_false() {
        fn aux<RHS: Bool>() {
            let _: ty::Val< False > = ty::Val::val::<And, ( False, RHS )>();
        }
        aux::<False>();
        aux::< True>();
    }

    #[test]
    fn and_true() {
        fn aux<RHS: Bool>() {
            let _: ty::Val< RHS > = ty::Val::val::<And, ( True, RHS )>();
        }
        aux::<False>();
        aux::< True>();
    }

    #[test]
    fn or_false() {
        fn aux<RHS: Bool>() {
            let _: ty::Val< RHS > = ty::Val::val::<Or, ( False, RHS )>();
        }
        aux::<False>();
        aux::< True>();
    }

    #[test]
    fn or_true() {
        fn aux<RHS: Bool>() {
            let _: ty::Val< True > = ty::Val::val::<Or, ( True, RHS )>();
        }
        aux::<False>();
        aux::< True>();
    }

    #[test]
    fn if_false() {
        fn aux<LHS, RHS>() {
            let _: ty::Val< RHS > = ty::Val::val::<If, ( False, LHS, RHS )>();
        }
        aux::<(), bool>();
        aux::<(), bool>();
    }

    #[test]
    fn if_true() {
        fn aux<LHS, RHS>() {
            let _: ty::Val< LHS > = ty::Val::val::<If, (  True, LHS, RHS )>();
        }
        aux::<(), bool>();
        aux::<(), bool>();
    }
}

