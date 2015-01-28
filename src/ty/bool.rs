use hlist::*;
use ty::{
    Rule,
    Sig,
    Tm,
    Ty,
};

/// Type-level booleans
pub enum
    Bool
{}

/// ```ignore
/// ----------
/// Bool :: Ty
/// ```
impl
    Ty
for
    Bool
{}

/// Type-level false
pub enum
    FF
{}

/// ```ignore
/// ---------
/// ff : Bool
/// ```
impl
    Tm<Bool>
for
    FF
{}

/// Type-level true
pub enum
    TT
{}

/// ```ignore
/// ---------
/// tt : Bool
/// ```
impl
    Tm<Bool>
for
    TT
{}



/// Type-level partial operation for bool negation
pub enum
    Not
{}

/// ```ignore
/// p : Bool
/// -------------
/// not(p) : Bool
/// ```
impl
    Sig
for
    Not
{
    type Dom = Bool;
    type Cod = Bool;
}

/// `not(ff) => tt`
impl
    Rule<Not>
for
    FF
{
    type Out = TT;
}

/// `not(tt) => ff`
impl
    Rule<Not>
for
    TT
{
    type Out = FF;
}



/// Type-level partial operation for bool conjunction
pub enum
    And
{}

/// ```ignore
/// p : Bool
/// q : Bool
/// ----------------
/// and(p, q) : Bool
/// ```
impl
    Sig
for
    And
{
    type Dom = HCons<Bool, HCons<Bool, HNil>>;
    type Cod = Bool;
}

/// `and(ff, q) => ff`
impl<
    B,
>
    Rule<And>
for
    HCons<FF, HCons<B, HNil>>
where
    B: Tm<Bool>,
{
    type Out = FF;
}

/// `and(tt, q) => q`
impl<
    B,
>
    Rule<And>
for
    HCons<TT, HCons<B, HNil>>
where
    B: Tm<Bool>,
{
    type Out = B;
}



/// Type-level partial operation for bool disjunction
pub enum
    Or
{}

/// ```ignore
/// p : Bool
/// q : Bool
/// ---------------
/// or(p, q) : Bool
/// ```
impl
    Sig
for
    Or
{
    type Dom = HCons<Bool, HCons<Bool, HNil>>;
    type Cod = Bool;
}

/// `or(ff, q) => q`
impl<
    B,
>
    Rule<Or>
for
    HCons<FF, HCons<B, HNil>>
where
    B: Tm<Bool>,
{
    type Out = B;
}

/// `or(tt, q) => tt`
impl<
    B,
>
    Rule<Or>
for
    HCons<TT, HCons<B, HNil>>
where
    B: Tm<Bool>,
{
    type Out = TT;
}



/// Type-level partial operation for bool conditional
pub enum
    If<A>
where
    A: Ty,
{}

/// ```ignore
/// A :: Ty
/// b : Bool
/// xt : A
/// xf : A
/// ------------------
/// if(b, xt, xf) : A
/// ```
impl<
    A,
>
    Sig
for
    If<A>
where
    A: Ty,
{
    type Dom = HCons<Bool, HCons<A, HCons<A, HNil>>>;
    type Cod = A;
}

/// `if(ff, xt, xf) => xf`
impl<
    A,
    B0,
    B1,
>
    Rule<If<A>>
for
    HCons<FF, HCons<B0, HCons<B1, HNil>>>
where
    A: Ty,
    B0: Tm<A>,
    B1: Tm<A>,
{
    type Out = B1;
}

/// `if(tt, xt, xf) => xt`
impl<
    A,
    B0,
    B1,
>
    Rule<If<A>>
for
    HCons<TT, HCons<B0, HCons<B1, HNil>>>
where
    A: Ty,
    B0: Tm<A>,
    B1: Tm<A>,
{
    type Out = B0;
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
            let x: Wit<HCons<FF, HCons<B1, HNil>>> = Wit;
            let _: Wit<FF> = x.app::<And>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn and_true() {
        fn aux<B1: Tm<Bool>>() {
            let x: Wit<HCons<TT, HCons<B1, HNil>>> = Wit;
            let _: Wit<B1> = x.app::<And>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn or_false() {
        fn aux<B1: Tm<Bool>>() {
            let x: Wit<HCons<FF, HCons<B1, HNil>>> = Wit;
            let _: Wit<B1> = x.app::<Or>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn or_true() {
        fn aux<B1: Tm<Bool>>() {
            let x: Wit<HCons<TT, HCons<B1, HNil>>> = Wit;
            let _: Wit<TT> = x.app::<Or>();
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn if_false() {
        let x: Wit<HCons<FF, HCons<FF, HCons<TT, HNil>>>> = Wit;
        let _: Wit<TT> = x.app::<If<Bool>>();
    }

    #[test]
    fn if_true() {
        let x: Wit<HCons<TT, HCons<FF, HCons<TT, HNil>>>> = Wit;
        let _: Wit<FF> = x.app::<If<Bool>>();
    }
}
