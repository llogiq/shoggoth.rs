use ty::fun;

/// Type-level `false`
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum False {}

/// Type-level `true`
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum True {}

/// Predicate classifying type-level `bool`
pub trait Bool {}
impl Bool for False {}
impl Bool for  True {}

/// Type-level function for `bool` negation
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Not {}
impl fun::Fn<(False,)> for Not { type O =  True; }
impl fun::Fn<( True,)> for Not { type O = False; }

/// Type-level function for `bool` conjunction
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum And {}
impl<RHS: Bool> fun::Fn<(False, RHS,)> for And { type O = False; }
impl<RHS: Bool> fun::Fn<( True, RHS,)> for And { type O =   RHS; }

/// Type-level function for `bool` disjunction
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Or {}
impl<RHS: Bool> fun::Fn<(False, RHS,)> for Or { type O =  RHS; }
impl<RHS: Bool> fun::Fn<( True, RHS,)> for Or { type O = True; }

/// Type-level function for `bool` conditional
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum If {}
impl<LHS, RHS> fun::Fn<( True, LHS, RHS,)> for If { type O = LHS; }
impl<LHS, RHS> fun::Fn<(False, LHS, RHS,)> for If { type O = RHS; }

#[cfg(test)]
mod tests {
    use ty::fun::{
        Val,
        val,
    };
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
    fn not_false() { let _: Val<True> = val::<Not, (False,)>(); }

    #[test]
    fn not_true () { let _: Val<False> = val::<Not, (True,)>(); }

    #[test]
    fn and_false() {
        fn aux<RHS: Bool>() {
            let _: Val<False> = val::<And, (False, RHS,)>();
        }
        aux::<False>();
        aux::< True>();
    }

    #[test]
    fn and_true() {
        fn aux<RHS: Bool>() {
            let _: Val< RHS > = val::<And, (True, RHS,)>();
        }
        aux::<False>();
        aux::< True>();
    }

    #[test]
    fn or_false() {
        fn aux<RHS: Bool>() {
            let _: Val<RHS> = val::<Or, (False, RHS,)>();
        }
        aux::<False>();
        aux::< True>();
    }

    #[test]
    fn or_true() {
        fn aux<RHS: Bool>() {
            let _: Val<True> = val::<Or, (True, RHS,)>();
        }
        aux::<False>();
        aux::< True>();
    }

    #[test]
    fn if_false() {
        fn aux<LHS, RHS>() {
            let _: Val<RHS> = val::<If, (False, LHS, RHS,)>();
        }
        aux::<(), bool>();
        aux::<(), bool>();
    }

    #[test]
    fn if_true() {
        fn aux<LHS, RHS>() {
            let _: Val<LHS> = val::<If, (True, LHS, RHS,)>();
        }
        aux::<(), bool>();
        aux::<(), bool>();
    }
}

