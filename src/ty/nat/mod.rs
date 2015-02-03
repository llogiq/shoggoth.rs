use hlist::*;
use self::pos::{
    Pos,
};
use ty::{
    _0,
    _1,
    Ar,
    Ar1,
    Eval,
    Infer,
    Tm,
    Ty,
    infer,
};

/// Type-level positive natural numbers (binary)
pub mod pos;

/// Type-level natural numbers (binary)
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
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
    P: Tm<Pos>,
>
    Tm<Nat>
for
    P
{}



/// Type-level successor for natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
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
    type Ty = Ar1<Nat, Nat>;
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
      P: Tm<Pos>,
    Rec: Tm<Nat>,
>
    Eval<Succ>
for
    HC<P, HN>
where
      HC<P, HN>: Eval<pos::Succ, Out = Rec>,
{
    type Out = Rec;
}
{
    type Out = Rec;
}



/// Type-level addition for natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
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
    type Ty = Ar<HC<Nat, HC<Nat, HN>>, Nat>;
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
      P0: Tm<Pos>,
>
    Eval<Add>
for
    HC<P0, HC<_0, HN>>
{
    type Out = P0;
}

/// `add<Nat>(p, q) ==> add<Pos>(p, q)`
impl<
      P0: Tm<Pos>,
      P1: Tm<Pos>,
     Rec: Tm<Nat>,
>
    Eval<Add>
for
    HC<P0, HC<P1, HN>>
where
    HC<P0, HC<P1, HN>>
        : Eval<pos::Add, Out = Rec>,
{
    type Out = Rec;
}



/// Type-level multiplication for natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
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
    type Ty = Ar<HC<Nat, HC<Nat, HN>>, Nat>;
}

/// `mul(0, n) ==> 0`
impl<
      P1: Tm<Pos>,
>
    Eval<Mul>
for
    HC<_0, HC<P1, HN>>
{
    type Out = _0;
}

/// `mul(m, 0) ==> 0`
impl<
      P0: Tm<Pos>,
>
    Eval<Mul>
for
    HC<P0, HC<_0, HN>>
{
    type Out = _0;
}

/// `mul<Nat>(p, q) ==> mul<Pos>(p, q)`
impl<
      P0: Tm<Pos>,
      P1: Tm<Pos>,
     Rec: Tm<Nat>,
>
    Eval<Mul>
for
    HC<P0, HC<P1, HN>>
where
    HC<P0, HC<P1, HN>>
        : Eval<pos::Mul, Out = Rec>,
{
    type Out = Rec;
}

#[cfg(test)]
mod test {
    use hlist::{
        HC,
        HN,
    };
    use super::*;
    use ty::*;

    // FIXME: add algebraic tests

    #[test]
    fn add_0() {
        let x0: Witness<Ap<Add, HC<_0b, HC<_16384b, HN>>>> = Witness;
        let x1: Witness<_16384b> = Witness;
        x0 == x1;
    }

    #[test]
    fn add() {
        let x0: Witness<Ap<Add, HC<_8192b, HC<_8192b, HN>>>> = Witness;
        let x1: Witness<_16384b> = Witness;
        x0 == x1;
    }

    #[test]
    fn mul_0() {
        let x0: Witness<Ap<Mul, HC<_0b, HC<_16384b, HN>>>> = Witness;
        let x1: Witness<_0b> = Witness;
        x0 == x1;
    }

    #[test]
    fn mul_1() {
        let x0: Witness<Ap<Mul, HC<_1b, HC<_16384b, HN>>>> = Witness;
        let x1: Witness<_16384b> = Witness;
        x0 == x1;
    }

    #[test]
    fn mul() {
        let x0: Witness<Ap<Mul, HC<_32b, HC<_2048b, HN>>>> = Witness;
        let x1: Witness<_65536b> = Witness;
        x0 == x1;
    }
}
