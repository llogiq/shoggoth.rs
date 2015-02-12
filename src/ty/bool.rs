use hlist::*;
use ty::{
    Eval,
    Infer,
    infer,
};

/// Type-level false
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FF {}

/// Type-level true
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TT {}

/// Type-level operation for bool negation
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Not {}

impl Infer for Not {
    type Arity = HC<(), HN>;
    type Mode = infer::mode::Constant;
}

/// `not(ff) ==> tt`
impl Eval<Not> for HC<FF, HN> {
    type Out = TT;
}

/// `not(tt) ==> ff`
impl Eval<Not> for HC<TT, HN> {
    type Out = FF;
}

/// Type-level operation for bool conjunction
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum And {}

impl Infer for And {
    type Arity = HC<(), HC<(), HN>>;
    type Mode = infer::mode::Constant;
}

/// `and(ff, b1) ==> ff`
impl<B1> Eval<And> for HC<FF, HC<B1, HN>> {
    type Out = FF;
}

/// `and(tt, b1) ==> b1`
impl<B1> Eval<And> for HC<TT, HC<B1, HN>> {
    type Out = B1;
}

/// Type-level operation for bool disjunction
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Or {}

impl Infer for Or {
    type Arity = HC<(), HC<(), HN>>;
    type Mode = infer::mode::Constant;
}

/// `or(ff, b1) ==> b1`
impl<B1> Eval<Or> for HC<FF, HC<B1, HN>> {
    type Out = B1;
}

/// `or(tt, b1) ==> tt`
impl<B1> Eval<Or> for HC<TT, HC<B1, HN>> {
    type Out = TT;
}

/// Type-level operation for bool conditional
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum If {}

impl Infer for If {
    type Arity = HC<(), HC<(), HC<(), HN>>>;
    type Mode = infer::mode::Constant;
}

/// `if(ff, m0, m1) ==> m1`
impl<M0, M1> Eval<If> for HC<FF, HC<M0, HC<M1, HN>>> {
    type Out = M1;
}

/// `if(tt, m0, m1) ==> m0`
impl<M0, M1> Eval<If> for HC<TT, HC<M0, HC<M1, HN>>> {
    type Out = M0;
}

#[cfg(test)]
mod test {
    use hlist::*;
    use ty::*;

    // FIXME: implement tests corresponding to boolean algebras

    #[test]
    fn not_false() {
        let x0 = Witness::<Ap1<Not, FF>>;
        let x1 = Witness::<TT>;
        x0 == x1;
    }

    #[test]
    fn not_true () {
        let x0 = Witness::<Ap1<Not, TT>>;
        let x1 = Witness::<FF>;
        x0 == x1;
    }

    #[test]
    fn and_false() {
        fn aux<B: Eq>() {
            let x0 = Witness::<Ap<And, HC<FF, HC<B, HN>>>>;
            let x1 = Witness::<FF>;
            x0 == x1;
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn and_true() {
        fn aux<B: Eq>() {
            let x0 = Witness::<Ap<And, HC<TT, HC<B, HN>>>>;
            let x1 = Witness::<B>;
            x0 == x1;
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn or_false() {
        fn aux<B: Eq>() {
            let x0 = Witness::<Ap<Or, HC<FF, HC<B, HN>>>>;
            let x1 = Witness::<B>;
            x0 == x1;
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn or_true() {
        fn aux<B: Eq>() {
            let x0 = Witness::<Ap<Or, HC<TT, HC<B, HN>>>>;
            let x1 = Witness::<TT>;
            x0 == x1;
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn if_false() {
        let x0 = Witness::<Ap<If, HC<FF, HC<FF, HC<TT, HN>>>>>;
        let x1 = Witness::<TT>;
        x0 == x1;
    }

    #[test]
    fn if_true() {
        let x0 = Witness::<Ap<If, HC<TT, HC<FF, HC<TT, HN>>>>>;
        let x1 = Witness::<FF>;
        x0 == x1;
    }
}
