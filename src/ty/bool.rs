use ty::{
    Tm,
    Ty,
    fun,
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
impl fun::Sig for Not { type Dom = (Bool,); type Cod = Bool; }
impl fun::Fn<Not> for (False,) { type O = True; }
impl fun::Fn<Not> for (True,) { type O = False; }

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
impl fun::Sig for And { type Dom = (Bool, Bool,); type Cod = Bool; }
impl<B: Tm<Bool>> fun::Fn<And> for (False, B,) { type O = False; }
impl<B: Tm<Bool>> fun::Fn<And> for (True, B,) { type O = B; }

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
impl fun::Sig for Or { type Dom = (Bool, Bool,); type Cod = Bool; }
impl<B: Tm<Bool>> fun::Fn<Or> for (False, B,) { type O = B; }
impl<B: Tm<Bool>> fun::Fn<Or> for (True, B,) { type O = True; }

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
impl<A: Ty> fun::Sig for If<A> { type Dom = (Bool, A, A,); type Cod = A; }
impl<A: Ty, B0: Tm<A>, B1: Tm<A>> fun::Fn<If<A>> for (False, B0, B1,) { type O = B1; }
impl<A: Ty, B0: Tm<A>, B1: Tm<A>> fun::Fn<If<A>> for (True, B0, B1,) { type O = B0; }

#[cfg(test)]
mod tests {
    use ty;
    use ty::bool::*;
    use ty::literal::*;
    use ty::wit::*;

    // FIXME: implement tests corresponding to boolean algebras

    #[test]
    fn not_false() { let _: Wit<TT> = wit::<Not, (FF,)>(); }

    #[test]
    fn not_true () { let _: Wit<FF> = wit::<Not, (TT,)>(); }

    #[test]
    fn and_false() {
        fn aux<B1: ty::Tm<Bool>>() {
            let _: Wit<FF> = wit::<And, (FF, B1,)>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn and_true() {
        fn aux<B1: ty::Tm<Bool>>() {
            let _: Wit<B1> = wit::<And, (TT, B1,)>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn or_false() {
        fn aux<B1: ty::Tm<Bool>>() {
            let _: Wit<B1> = wit::<Or, (FF, B1,)>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn or_true() {
        fn aux<B1: ty::Tm<Bool>>() {
            let _: Wit<TT> = wit::<Or, (TT, B1,)>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn if_false() {
        let _: Wit<TT> = wit::<If<Bool>, (FF, FF, TT,)>();
    }

    #[test]
    fn if_true() {
        let _: Wit<FF> = wit::<If<Bool>, (TT, FF, TT,)>();
    }
}
