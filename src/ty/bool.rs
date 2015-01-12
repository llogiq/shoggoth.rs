use ty;

/// Type-level boolean `false`
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum False {}

/// Type-level boolean `true`
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum True {}

/// Predicate classifying type-level booleans
pub trait Bool {}
impl Bool for False {}
impl Bool for True {}

/// Type-level function for boolean negation
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Not {}
impl ty::fun::Fn<(False,)> for Not { type O = True; }
impl ty::fun::Fn<(True,)> for Not { type O = False; }

/// Type-level function for boolean conjunction
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum And {}
impl<RHS: Bool> ty::fun::Fn<(False, RHS,)> for And { type O = False; }
impl<RHS: Bool> ty::fun::Fn<(True, RHS,)> for And { type O = RHS; }

/// Type-level function for boolean disjunction
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Or {}
impl<RHS: Bool> ty::fun::Fn<(False, RHS,)> for Or { type O = RHS; }
impl<RHS: Bool> ty::fun::Fn<(True, RHS,)> for Or { type O = True; }

/// Type-level function for boolean conditional
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum If {}
impl<LHS, RHS> ty::fun::Fn<(True, LHS, RHS,)> for If { type O = LHS; }
impl<LHS, RHS> ty::fun::Fn<(False, LHS, RHS,)> for If { type O = RHS; }

#[cfg(test)]
mod tests {
    use ty::bool::*;
    use ty::literal::*;
    use ty::val::*;

    // FIXME: implement tests corresponding to boolean algebras

    #[test]
    fn not_false() { let _: Val<TT> = val::<Not, (FF,)>(); }

    #[test]
    fn not_true () { let _: Val<FF> = val::<Not, (TT,)>(); }

    #[test]
    fn and_false() {
        fn aux<RHS: Bool>() {
            let _: Val<FF> = val::<And, (FF, RHS,)>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn and_true() {
        fn aux<RHS: Bool>() {
            let _: Val<RHS> = val::<And, (TT, RHS,)>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn or_false() {
        fn aux<RHS: Bool>() {
            let _: Val<RHS> = val::<Or, (FF, RHS,)>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn or_true() {
        fn aux<RHS: Bool>() {
            let _: Val<TT> = val::<Or, (TT, RHS,)>();
        }
        aux::<FF>();
        aux::< TT>();
    }

    #[test]
    fn if_false() {
        fn aux<LHS, RHS>() {
            let _: Val<RHS> = val::<If, (FF, LHS, RHS,)>();
        }
        aux::<(), bool>();
        aux::<(), bool>();
    }

    #[test]
    fn if_true() {
        fn aux<LHS, RHS>() {
            let _: Val<LHS> = val::<If, (TT, LHS, RHS,)>();
        }
        aux::<(), bool>();
        aux::<(), bool>();
    }
}

