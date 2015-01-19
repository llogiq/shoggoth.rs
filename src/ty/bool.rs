use hlist::{
    HC,
    HN,
};
use ty::{
    FnTm,
    Sig,
    Tm,
    Ty,
};

/// Type-level bool
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Bool {}
impl Ty for Bool {}

/// Type-level false
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum False {}
impl Tm<Bool> for False {}

/// Type-level true
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum True {}
impl Tm<Bool> for True {}

/// Type-level function for bool negation
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Not {}
impl Sig for Not { type Dom = Bool; type Cod = Bool; }
impl FnTm<Not> for False { type O = True; }
impl FnTm<Not> for True { type O = False; }

/// Type-level function for bool conjunction
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum And {}
impl Sig for And { type Dom = HC<Bool, HC<Bool, HN>>; type Cod = Bool; }
impl<B: Tm<Bool>> FnTm<And> for HC<False, HC<B, HN>> { type O = False; }
impl<B: Tm<Bool>> FnTm<And> for HC<True, HC<B, HN>> { type O = B; }

/// Type-level function for bool disjunction
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Or {}
impl Sig for Or { type Dom = HC<Bool, HC<Bool, HN>>; type Cod = Bool; }
impl<B: Tm<Bool>> FnTm<Or> for HC<False, HC<B, HN>> { type O = B; }
impl<B: Tm<Bool>> FnTm<Or> for HC<True, HC<B, HN>> { type O = True; }

/// Type-level function for bool conditional
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum If<A: Ty> {}
impl<A: Ty> Sig for If<A> { type Dom = HC<Bool, HC<A, HC<A, HN>>>; type Cod = A; }
impl<A: Ty, B0: Tm<A>, B1: Tm<A>> FnTm<If<A>> for HC<False, HC<B0, HC<B1, HN>>> { type O = B1; }
impl<A: Ty, B0: Tm<A>, B1: Tm<A>> FnTm<If<A>> for HC<True, HC<B0, HC<B1, HN>>> { type O = B0; }

#[cfg(test)]
mod tests {
    use ty;
    use ty::bool::*;
    use ty::literal::*;
    use ty::wit::*;

    // FIXME: implement tests corresponding to boolean algebras

    #[test]
    fn not_false() { let _: Wit<TT> = wit::<Not, FF>(); }

    #[test]
    fn not_true () { let _: Wit<FF> = wit::<Not, TT>(); }

    #[test]
    fn and_false() {
        fn aux<B1: ty::Tm<Bool>>() {
            let _: Wit<FF> = wit::<And, HC<FF, HC<B1, HN>>>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn and_true() {
        fn aux<B1: ty::Tm<Bool>>() {
            let _: Wit<B1> = wit::<And, HC<TT, HC<B1, HN>>>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn or_false() {
        fn aux<B1: ty::Tm<Bool>>() {
            let _: Wit<B1> = wit::<Or, HC<FF, HC<B1, HN>>>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn or_true() {
        fn aux<B1: ty::Tm<Bool>>() {
            let _: Wit<TT> = wit::<Or, HC<TT, HC<B1, HN>>>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn if_false() {
        let _: Wit<TT> = wit::<If<Bool>, HC<FF, HC<FF, HC<TT, HN>>>>();
    }

    #[test]
    fn if_true() {
        let _: Wit<FF> = wit::<If<Bool>, HC<TT, HC<FF, HC<TT, HN>>>>();
    }
}
