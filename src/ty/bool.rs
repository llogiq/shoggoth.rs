use hlist::*;
use ty::{
    Rule,
    Sig,
    Tm,
    Ty,
};

/// Type-level booleans
pub enum Bool {}

/// ```
/// ----------
/// Bool :: Ty
/// ```
impl Ty for Bool {}

/// Type-level false
pub enum FF {}

/// ```
/// ---------
/// ff : Bool
/// ```
impl Tm<Bool> for FF {}

/// Type-level true
pub enum TT {}

/// ```
/// ---------
/// tt : Bool
/// ```
impl Tm<Bool> for TT {}



/// Type-level partial operation for bool negation
pub enum Not {}

/// ```
/// p : Bool
/// -------------
/// not(p) : Bool
/// ```
impl Sig for Not {
    type Dom = Bool;
    type Cod = Bool;
}

/// `not(ff) => tt`
impl Rule<Not> for FF {
    type O = TT;
}

/// `not(tt) => ff`
impl Rule<Not> for TT {
    type O = FF;
}



/// Type-level partial operation for bool conjunction
pub enum And {}

/// ```
/// p : Bool
/// q : Bool
/// ----------------
/// and(p, q) : Bool
/// ```
impl Sig for And {
    type Dom = HC<Bool, HC<Bool, HN>>;
    type Cod = Bool;
}

/// `and(ff, q) => ff`
impl<B> Rule<And> for HC<FF, HC<B, HN>> where
    B: Tm<Bool>,
{
    type O = FF;
}

/// `and(tt, q) => q`
impl<B> Rule<And> for HC<TT, HC<B, HN>> where
    B: Tm<Bool>,
{
    type O = B;
}



/// Type-level partial operation for bool disjunction
pub enum Or {}

/// ```
/// p : Bool
/// q : Bool
/// ---------------
/// or(p, q) : Bool
/// ```
impl Sig for Or {
    type Dom = HC<Bool, HC<Bool, HN>>;
    type Cod = Bool;
}

/// `or(ff, q) => q`
impl<B> Rule<Or> for HC<FF, HC<B, HN>> where
    B: Tm<Bool>,
{
    type O = B;
}

/// `or(tt, q) => tt`
impl<B> Rule<Or> for HC<TT, HC<B, HN>> where
    B: Tm<Bool>,
{
    type O = TT;
}



/// Type-level partial operation for bool conditional
pub enum If<A: Ty> {}

/// ```
/// A :: Ty
/// b : Bool
/// xt : A
/// xf : A
/// ------------------
/// if(b, xt, xf) : A
/// ```
impl<A: Ty> Sig for If<A> {
    type Dom = HC<Bool, HC<A, HC<A, HN>>>;
    type Cod = A;
}

/// `if(ff, xt, xf) => xf`
impl<A, B0, B1> Rule<If<A>> for HC<FF, HC<B0, HC<B1, HN>>> where
    A: Ty,
    B0: Tm<A>,
    B1: Tm<A>,
{
    type O = B1;
}

/// `if(tt, xt, xf) => xt`
impl<A, B0, B1> Rule<If<A>> for HC<TT, HC<B0, HC<B1, HN>>> where
    A: Ty,
    B0: Tm<A>,
    B1: Tm<A>,
{
    type O = B0;
}



#[cfg(test)]
mod test {
    use hlist::*;
    use ty::*;

    // FIXME: implement tests corresponding to boolean algebras

    #[test]
    fn not_false() {
        let x: Wit<FF> = Wit;
        let _: Wit<TT> = x.app::<Not>();
    }

    #[test]
    fn not_true () {
        let x: Wit<TT> = Wit;
        let _: Wit<FF> = x.app::<Not>();
    }

    #[test]
    fn and_false() {
        fn aux<B1: Tm<Bool>>() {
            let x: Wit<HC<FF, HC<B1, HN>>> = Wit;
            let _: Wit<FF> = x.app::<And>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn and_true() {
        fn aux<B1: Tm<Bool>>() {
            let x: Wit<HC<TT, HC<B1, HN>>> = Wit;
            let _: Wit<B1> = x.app::<And>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn or_false() {
        fn aux<B1: Tm<Bool>>() {
            let x: Wit<HC<FF, HC<B1, HN>>> = Wit;
            let _: Wit<B1> = x.app::<Or>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn or_true() {
        fn aux<B1: Tm<Bool>>() {
            let x: Wit<HC<TT, HC<B1, HN>>> = Wit;
            let _: Wit<TT> = x.app::<Or>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn if_false() {
        let x: Wit<HC<FF, HC<FF, HC<TT, HN>>>> = Wit;
        let _: Wit<TT> = x.app::<If<Bool>>();
    }

    #[test]
    fn if_true() {
        let x: Wit<HC<TT, HC<FF, HC<TT, HN>>>> = Wit;
        let _: Wit<FF> = x.app::<If<Bool>>();
    }
}
