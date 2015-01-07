use ty::fun;

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
impl fun::Fn<Not, ( False, )> for fun::Call { type T =  True; }
impl fun::Fn<Not, (  True, )> for fun::Call { type T = False; }

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum And {}
impl<RHS: Bool> fun::Fn<And, ( False, RHS, )> for fun::Call { type T = False; }
impl<RHS: Bool> fun::Fn<And, (  True, RHS, )> for fun::Call { type T =   RHS; }

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Or {}
impl<RHS: Bool> fun::Fn<Or, ( False, RHS, )> for fun::Call { type T =  RHS; }
impl<RHS: Bool> fun::Fn<Or, (  True, RHS, )> for fun::Call { type T = True; }

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum If {}
impl<LHS, RHS> fun::Fn<If, (  True, LHS, RHS, )> for fun::Call { type T = LHS; }
impl<LHS, RHS> fun::Fn<If, ( False, LHS, RHS, )> for fun::Call { type T = RHS; }

#[cfg(test)]
mod tests {
    use ty::fun::Val;
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
    fn not_false() { let _: Val<  True > = Val::val::<Not, ( False, )>(); }

    #[test]
    fn not_true () { let _: Val< False > = Val::val::<Not, (  True, )>(); }

    #[test]
    fn and_false() {
        fn aux<RHS: Bool>() {
            let _: Val< False > = Val::val::<And, ( False, RHS, )>();
        }
        aux::<False>();
        aux::< True>();
    }

    #[test]
    fn and_true() {
        fn aux<RHS: Bool>() {
            let _: Val< RHS > = Val::val::<And, ( True, RHS, )>();
        }
        aux::<False>();
        aux::< True>();
    }

    #[test]
    fn or_false() {
        fn aux<RHS: Bool>() {
            let _: Val< RHS > = Val::val::<Or, ( False, RHS, )>();
        }
        aux::<False>();
        aux::< True>();
    }

    #[test]
    fn or_true() {
        fn aux<RHS: Bool>() {
            let _: Val< True > = Val::val::<Or, ( True, RHS, )>();
        }
        aux::<False>();
        aux::< True>();
    }

    #[test]
    fn if_false() {
        fn aux<LHS, RHS>() {
            let _: Val< RHS > = Val::val::<If, ( False, LHS, RHS, )>();
        }
        aux::<(), bool>();
        aux::<(), bool>();
    }

    #[test]
    fn if_true() {
        fn aux<LHS, RHS>() {
            let _: Val< LHS > = Val::val::<If, (  True, LHS, RHS, )>();
        }
        aux::<(), bool>();
        aux::<(), bool>();
    }
}

