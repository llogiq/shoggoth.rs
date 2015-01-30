use hlist::*;
use self::pos::{
    Pos,
};
use ty::{
    Rule,
    Sig,
    Tm,
    Ty,
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
    Sig
for
    Succ
{
    type Dom = Nat;
    type Cod = Nat;
}

/// `succ(0) ==> 1`
impl
    Rule<Succ>
for
    _0
{
    type Out = _1;
}

/// `succ<Nat>(p) ==> succ<Pos>(p)`
impl<
    Rec,
    P,
>
    Rule<Succ>
for
    P
where
    P: Rule<pos::Succ, Out = Rec>,
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
    Sig
for
    Add
{
    type Dom = HCons<Nat, HCons<Nat, HNil>>;
    type Cod = Nat;
}

/// `add(0, n) ==> n`
impl<
    P1,
>
    Rule<Add>
for
    HCons<_0, HCons<P1, HNil>>
where
    P1: Tm<Pos>,
{
    type Out = P1;
}

/// `add(m, 0) ==> m`
impl<
    P0,
>
    Rule<Add>
for
    HCons<P0, HCons<_0, HNil>>
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
    Rule<Add>
for
    HCons<P0, HCons<P1, HNil>>
where
    HCons<P0, HCons<P1, HNil>>: Rule<pos::Add, Out = Rec>,
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
    Sig
for
    Mul
{
    type Dom = HCons<Nat, HCons<Nat, HNil>>;
    type Cod = Nat;
}

/// `mul(0, n) ==> 0`
impl<
    P1,
>
    Rule<Mul>
for
    HCons<_0, HCons<P1, HNil>>
where
    P1: Tm<Pos>,
{
    type Out = _0;
}

/// `mul(m, 0) ==> 0`
impl<
    P0,
>
    Rule<Mul>
for
    HCons<P0, HCons<_0, HNil>>
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
    Rule<Mul>
for
    HCons<P0, HCons<P1, HNil>>
where
    HCons<P0, HCons<P1, HNil>>: Rule<pos::Mul, Out = Rec>,
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
        let x: Wit<HCons<_0b, HCons<_16384b, HNil>>> = Wit;
        let _: Wit<_16384b> = x.app::<Add>();
    }

    #[test]
    fn add() {
        let x: Wit<HCons<_8192b, HCons<_8192b, HNil>>> = Wit;
        let _: Wit<_16384b> = x.app::<Add>();
    }

    #[test]
    fn mul_0() {
        let x: Wit<HCons<_0b, HCons<_16384b, HNil>>> = Wit;
        let _: Wit<_0b> = x.app::<Mul>();
    }

    #[test]
    fn mul_1() {
        let x: Wit<HCons<_1b, HCons<_16384b, HNil>>> = Wit;
        let _: Wit<_16384b> = x.app::<Mul>();
    }

    #[test]
    fn mul() {
        let x: Wit<HCons<_32b, HCons<_2048b, HNil>>> = Wit;
        let _: Wit<_65536b> = x.app::<Mul>();
    }
}
