use ty::{
    Tm,
    Ty,
    fun,
};

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

/// Type-level boolean "false"
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

/// Type-level boolean "true"
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
impl fun::Sig for Not { type Dom = (Bool,); type Cod = Bool; }
impl fun::Fn<Not> for (False,) { type O = True; }
impl fun::Fn<Not> for (True,) { type O = False; }

// Type-level function for boolean conjunction
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
impl<B1: Tm<Bool>> fun::Fn<And> for (False, B1,) { type O = False; }
impl<B1: Tm<Bool>> fun::Fn<And> for (True, B1,) { type O = B1; }

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
impl fun::Sig for Or { type Dom = (Bool, Bool,); type Cod = Bool; }
impl<B1: Tm<Bool>> fun::Fn<Or> for (False, B1,) { type O = B1; }
impl<B1: Tm<Bool>> fun::Fn<Or> for (True, B1,) { type O = True; }

/// Type-level function for boolean conditional
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
    use ty::val::*;

    // FIXME: implement tests corresponding to boolean algebras

    #[test]
    fn not_false() { let _: Val<TT> = val::<Not, (FF,)>(); }

    #[test]
    fn not_true () { let _: Val<FF> = val::<Not, (TT,)>(); }

    #[test]
    fn and_false() {
        fn aux<B1: ty::Tm<Bool>>() {
            let _: Val<FF> = val::<And, (FF, B1,)>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn and_true() {
        fn aux<B1: ty::Tm<Bool>>() {
            let _: Val<B1> = val::<And, (TT, B1,)>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn or_false() {
        fn aux<B1: ty::Tm<Bool>>() {
            let _: Val<B1> = val::<Or, (FF, B1,)>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn or_true() {
        fn aux<B1: ty::Tm<Bool>>() {
            let _: Val<TT> = val::<Or, (TT, B1,)>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn if_false() {
        let _: Val<TT> = val::<If<Bool>, (FF, FF, TT,)>();
    }

    #[test]
    fn if_true() {
        let _: Val<FF> = val::<If<Bool>, (TT, FF, TT,)>();
    }
}
