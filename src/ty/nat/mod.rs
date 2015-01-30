use hlist::*;
use self::pos::{
    Pos,
};
use ty::{
    Arr,
    Arr1,
    Eval,
    Infer,
    Tm,
    Ty,
    infer,
};
use ty::bit::{
    _0,
    _1,
};

/// Type-level positive natural numbers (binary)
pub mod pos;

/// Type-level natural numbers (binary)
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum
    Nat
{}

/// ```ignore
/// ---------
/// Nat :: Ty
/// ```
impl
    Ty
for
    Nat
{}

/// ```ignore
/// -------
/// 0 : Nat
/// ```
impl
    Tm<Nat>
for
    _0
{}

/// ```ignore
/// p : Pos
/// -------
/// p : Nat
/// ```
impl<
    P,
>
    Tm<Nat>
for
    P
where
    P: Tm<Pos>,
{}



/// Type-level successor for natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum
    Succ
{}

/// ```ignore
/// n : Nat
/// -------------
/// succ(n) : Nat
/// ```
impl
    Infer
for
    Succ
{
    type Mode = infer::mode::Constant;
    type Ty = Arr1<Nat, Nat>;
}

/// `succ(0) ==> 1`
impl
    Eval<Succ>
for
    HC<_0, HN>
{
    type Out = _1;
}

/// `succ<Nat>(p) ==> succ<Pos>(p)`
impl<
    Rec,
    P,
>
    Eval<Succ>
for
    HC<P, HN>
where
    P: Eval<pos::Succ, Out = Rec>,
    P: Tm<Pos>,
    Rec: Tm<Nat>,
{
    type Out = Rec;
}



/// Type-level addition for natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum
    Add
{}

/// ```ignore
/// m : Nat
/// n : Nat
/// ---------------
/// add(m, n) : Nat
/// ```
impl
    Infer
for
    Add
{
    type Mode = infer::mode::Constant;
    type Ty = Arr<HC<Nat, HC<Nat, HN>>, Nat>;
}

/// `add(0, n) ==> n`
impl<
    P1,
>
    Eval<Add>
for
    HC<_0, HC<P1, HN>>
where
    P1: Tm<Pos>,
{
    type Out = P1;
}

/// `add(m, 0) ==> m`
impl<
    P0,
>
    Eval<Add>
for
    HC<P0, HC<_0, HN>>
where
    P0: Tm<Pos>,
{
    type Out = P0;
}

/// `add<Nat>(p, q) ==> add<Pos>(p, q)`
impl<
    P0,
    P1,
    Rec,
>
    Eval<Add>
for
    HC<P0, HC<P1, HN>>
where
    HC<P0, HC<P1, HN>>: Eval<pos::Add, Out = Rec>,
    P0: Tm<Pos>,
    P1: Tm<Pos>,
    Rec: Tm<Nat>,
{
    type Out = Rec;
}



/// Type-level multiplication for natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum
    Mul
{}

/// ```ignore
/// m : Nat
/// n : Nat
/// ---------------
/// mul(m, n) : Nat
/// ```
impl
    Infer
for
    Mul
{
    type Mode = infer::mode::Constant;
    type Ty = Arr<HC<Nat, HC<Nat, HN>>, Nat>;
}

/// `mul(0, n) ==> 0`
impl<
    P1,
>
    Eval<Mul>
for
    HC<_0, HC<P1, HN>>
where
    P1: Tm<Pos>,
{
    type Out = _0;
}

/// `mul(m, 0) ==> 0`
impl<
    P0,
>
    Eval<Mul>
for
    HC<P0, HC<_0, HN>>
where
    P0: Tm<Pos>,
{
    type Out = _0;
}

/// `mul<Nat>(p, q) ==> mul<Pos>(p, q)`
impl<
    P0,
    P1,
    Rec,
>
    Eval<Mul>
for
    HC<P0, HC<P1, HN>>
where
    HC<P0, HC<P1, HN>>: Eval<pos::Mul, Out = Rec>,
    P0: Tm<Pos>,
    P1: Tm<Pos>,
    Rec: Tm<Nat>,
{
    type Out = Rec;
}

#[cfg(test)]
mod test {
    use hlist::*;
    use super::*;
    use ty::*;

    // FIXME: add algebraic tests

    #[test]
    fn add_0() {
        let x0: Wit<Ap<Add, HC<_0b, HC<_16384b, HN>>>> = Wit;
        let x1: Wit<_16384b> = Wit;
        x0 == x1;
    }

    #[test]
    fn add() {
        let x0: Wit<Ap<Add, HC<_8192b, HC<_8192b, HN>>>> = Wit;
        let x1: Wit<_16384b> = Wit;
        x0 == x1;
    }

    #[test]
    fn mul_0() {
        let x0: Wit<Ap<Mul, HC<_0b, HC<_16384b, HN>>>> = Wit;
        let x1: Wit<_0b> = Wit;
        x0 == x1;
    }

    #[test]
    fn mul_1() {
        let x0: Wit<Ap<Mul, HC<_1b, HC<_16384b, HN>>>> = Wit;
        let x1: Wit<_16384b> = Wit;
        x0 == x1;
    }

    #[test]
    fn mul() {
        let x0: Wit<Ap<Mul, HC<_32b, HC<_2048b, HN>>>> = Wit;
        let x1: Wit<_65536b> = Wit;
        x0 == x1;
    }
}
