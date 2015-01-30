use hlist::*;
use ty::{
    Arr,
    Arr1,
    Eval,
    Infer,
    Tm,
    Ty,
    infer,
};

/// Type-level booleans
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
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
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
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
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
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
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum
    Not
{}

/// ```ignore
/// b : Bool
/// -------------
/// not(b) : Bool
/// ```
impl
    Infer
for
    Not
{
    type Mode = infer::mode::Constant;
    type Ty = Arr1<Bool, Bool>;
}

/// `not(ff) ==> tt`
impl
    Eval<Not>
for
    HC<FF, HN>
{
    type Out = TT;
}

/// `not(tt) ==> ff`
impl
    Eval<Not>
for
    HC<TT, HN>
{
    type Out = FF;
}



/// Type-level partial operation for bool conjunction
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum
    And
{}

/// ```ignore
/// b0 : Bool
/// b1 : Bool
/// ------------------
/// and(b0, b1) : Bool
/// ```
impl
    Infer
for
    And
{
    type Mode = infer::mode::Constant;
    type Ty = Arr<HC<Bool, HC<Bool, HN>>, Bool>;
}

/// `and(ff, b1) ==> ff`
impl<
    B1,
>
    Eval<And>
for
    HC<FF, HC<B1, HN>>
where
    B1: Tm<Bool>,
{
    type Out = FF;
}

/// `and(tt, b1) ==> b1`
impl<
    B1,
>
    Eval<And>
for
    HC<TT, HC<B1, HN>>
where
    B1: Tm<Bool>,
{
    type Out = B1;
}



/// Type-level partial operation for bool disjunction
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum
    Or
{}

/// ```ignore
/// b0 : Bool
/// b1 : Bool
/// -----------------
/// or(b0, b1) : Bool
/// ```
impl
    Infer
for
    Or
{
    type Mode = infer::mode::Constant;
    type Ty = Arr<HC<Bool, HC<Bool, HN>>, Bool>;
}

/// `or(ff, b1) ==> b1`
impl<
    B1,
>
    Eval<Or>
for
    HC<FF, HC<B1, HN>>
where
    B1: Tm<Bool>,
{
    type Out = B1;
}

/// `or(tt, b1) ==> tt`
impl<
    B1,
>
    Eval<Or>
for
    HC<TT, HC<B1, HN>>
where
    B1: Tm<Bool>,
{
    type Out = TT;
}



/// Type-level partial operation for bool conditional
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum
    If<A>
where
    A: Ty,
{}

/// ```ignore
/// A :: Ty
/// b : Bool
/// m0 : A
/// m1 : A
/// -----------------
/// if(b, m0, m1) : A
/// ```
impl<
    A,
>
    Infer
for
    If<A>
where
    A: Ty,
{
    type Mode = infer::mode::Constant;
    type Ty = Arr<HC<Bool, HC<A, HC<A, HN>>>, A>;
}

/// `if(ff, m0, m1) ==> m1`
impl<
    A,
    M0,
    M1,
>
    Eval<If<A>>
for
    HC<FF, HC<M0, HC<M1, HN>>>
where
    A: Ty,
    M0: Tm<A>,
    M1: Tm<A>,
{
    type Out = M1;
}

/// `if(tt, m0, m1) ==> m0`
impl<
    A,
    M0,
    M1,
>
    Eval<If<A>>
for
    HC<TT, HC<M0, HC<M1, HN>>>
where
    A: Ty,
    M0: Tm<A>,
    M1: Tm<A>,
{
    type Out = M0;
}



#[cfg(test)]
mod test {
    use hlist::*;
    use ty::*;

    // FIXME: implement tests corresponding to boolean algebras

    #[test]
    fn not_false() {
        let x0: Wit<Ap1<Not, FF>> = Wit;
        let x1: Wit<TT> = Wit;
        x0 == x1;
    }

    #[test]
    fn not_true () {
        let x0: Wit<Ap1<Not, TT>> = Wit;
        let x1: Wit<FF> = Wit;
        x0 == x1;
    }

    #[test]
    fn and_false() {
        fn aux<B: Tm<Bool> + Eq>() {
            let x0: Wit<Ap<And, HC<FF, HC<B, HN>>>> = Wit;
            let x1: Wit<FF> = Wit;
            x0 == x1;
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn and_true() {
        fn aux<B: Tm<Bool> + Eq>() {
            let x0: Wit<Ap<And, HC<TT, HC<B, HN>>>> = Wit;
            let x1: Wit<B> = Wit;
            x0 == x1;
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn or_false() {
        fn aux<B: Tm<Bool> + Eq>() {
            let x0: Wit<Ap<Or, HC<FF, HC<B, HN>>>> = Wit;
            let x1: Wit<B> = Wit;
            x0 == x1;
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn or_true() {
        fn aux<B: Tm<Bool> + Eq>() {
            let x0: Wit<Ap<Or, HC<TT, HC<B, HN>>>> = Wit;
            let x1: Wit<TT> = Wit;
            x0 == x1;
        }
        aux::<FF>();
        aux::<TT>();
    }

    #[test]
    fn if_false() {
        let x0: Wit<Ap<If<Bool>, HC<FF, HC<FF, HC<TT, HN>>>>> = Wit;
        let x1: Wit<TT> = Wit;
        x0 == x1;
    }

    #[test]
    fn if_true() {
        let x0: Wit<Ap<If<Bool>, HC<TT, HC<FF, HC<TT, HN>>>>> = Wit;
        let x1: Wit<FF> = Wit;
        x0 == x1;
    }
}
