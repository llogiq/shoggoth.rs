use hlist::*;
use ty::{
    Rule,
    Sig,
    Tm,
    Ty,
};

/// Type-level bool
pub enum Bool {}

impl Ty for Bool {}

/// Type-level false
pub enum FF {}

impl Tm<Bool> for FF {}

/// Type-level true
pub enum TT {}

impl Tm<Bool> for TT {}

/// Type-level function for bool negation


pub enum Not {}
impl Sig for Not { type Dom = Bool; type Cod = Bool; }
impl Rule<Not> for FF { type O = TT; }
impl Rule<Not> for TT { type O = FF; }

/// Type-level function for bool conjunction





pub enum And {}
impl Sig for And { type Dom = HC<Bool, HC<Bool, HN>>; type Cod = Bool; }
impl<B: Tm<Bool>> Rule<And> for HC<FF, HC<B, HN>> { type O = FF; }
impl<B: Tm<Bool>> Rule<And> for HC<TT, HC<B, HN>> { type O = B; }

/// Type-level function for bool disjunction





pub enum Or {}
impl Sig for Or { type Dom = HC<Bool, HC<Bool, HN>>; type Cod = Bool; }
impl<B: Tm<Bool>> Rule<Or> for HC<FF, HC<B, HN>> { type O = B; }
impl<B: Tm<Bool>> Rule<Or> for HC<TT, HC<B, HN>> { type O = TT; }

/// Type-level function for bool conditional





pub enum If<A: Ty> {}
impl<A: Ty> Sig for If<A> { type Dom = HC<Bool, HC<A, HC<A, HN>>>; type Cod = A; }
impl<A: Ty, B0: Tm<A>, B1: Tm<A>> Rule<If<A>> for HC<FF, HC<B0, HC<B1, HN>>> { type O = B1; }
impl<A: Ty, B0: Tm<A>, B1: Tm<A>> Rule<If<A>> for HC<TT, HC<B0, HC<B1, HN>>> { type O = B0; }




#[cfg(test)]
mod test {
    use hlist::*;
    use ty::*;

    // FIXME: implement tests corresponding to boolean algebras

    #[test]
    fn not_false() { let _: Wit<TT> = app::<Not, FF>(); }

    #[test]
    fn not_true () { let _: Wit<FF> = app::<Not, TT>(); }

    #[test]
    fn and_false() {
        fn aux<B1: Tm<Bool>>() {
            let _: Wit<FF> = app::<And, HC<FF, HC<B1, HN>>>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn and_true() {
        fn aux<B1: Tm<Bool>>() {
            let _: Wit<B1> = app::<And, HC<TT, HC<B1, HN>>>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn or_false() {
        fn aux<B1: Tm<Bool>>() {
            let _: Wit<B1> = app::<Or, HC<FF, HC<B1, HN>>>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn or_true() {
        fn aux<B1: Tm<Bool>>() {
            let _: Wit<TT> = app::<Or, HC<TT, HC<B1, HN>>>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn if_false() {
        let _: Wit<TT> = app::<If<Bool>, HC<FF, HC<FF, HC<TT, HN>>>>();
    }

    #[test]
    fn if_true() {
        let _: Wit<FF> = app::<If<Bool>, HC<TT, HC<FF, HC<TT, HN>>>>();
    }
}
