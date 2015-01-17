use self::pos::{
    Pos,
};
use ty::bit::*;
use ty::{
    Tm,
    Ty,
    fun,
};

/// Binary positive natural numbers
pub mod pos;

/// Binary natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Nat {}
impl Ty for Nat {}

impl Tm<Nat> for _0 {}
impl<P: Tm<Pos>> Tm<Nat> for P {}

/// Type-level successor for binary natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Succ {}
impl fun::Sig for Succ { type Dom = (Nat,); type Cod = Nat; }
// 0 => 1
impl fun::Fn<Succ> for (_0,)
{
    type O = _1;
}
// p => succ(p)
impl<P: Tm<Pos>, Rec: Tm<Nat>> fun::Fn<Succ> for (P,) where
    (P,): fun::Fn<pos::Succ, O = Rec>,
{
    type O = Rec;
}

/// Type-level addition for binary natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Add {}
impl fun::Sig for Add { type Dom = (Nat, Nat,); type Cod = Nat; }
impl<P1: Tm<Pos>> fun::Fn<Add> for ((_0), (P1),)
// 0, n => n
{
    type O = P1;
}
impl<P0: Tm<Pos>> fun::Fn<Add> for (P0, (_0),)
// m, 0 => m
{
    type O = P0;
}
// p, q => p + q
impl<P0: Tm<Pos>, P1: Tm<Pos>, Rec: Tm<Nat>> fun::Fn<Add> for ((P0), (P1),) where
    ((P0), (P1),): fun::Fn<pos::Add, O = Rec>,
{
    type O = Rec;
}

/// Type-level multiplication for binary natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Mul {}
impl fun::Sig for Mul { type Dom = (Nat, Nat,); type Cod = Nat; }
// 0, n => 0
impl<P1: Tm<Pos>> fun::Fn<Mul> for ((_0), (P1),)
{
    type O = _0;
}
// m, 0 => 0
impl<P0: Tm<Pos>> fun::Fn<Mul> for (P0, (_0),)
{
    type O = _0;
}
// p, q => p * q
impl<P0: Tm<Pos>, P1: Tm<Pos>, Rec: Tm<Nat>> fun::Fn<Mul> for ((P0), (P1),) where
    ((P0), (P1),): fun::Fn<pos::Mul, O = Rec>,
{
    type O = Rec;
}

#[cfg(test)]
mod tests {
    use super::*;
    use ty::literal::*;
    use ty::val::*;

    // FIXME: add algebraic tests

    #[test]
    fn add_0() { let _: Val<_16384b> = val::<Add, (_0b, _16384b,)>(); }

    #[test]
    fn add() { let _: Val<_16384b> = val::<Add, (_8192b, _8192b,)>(); }

    #[test]
    fn mul_0() { let _: Val<_0b> = val::<Mul, (_0b, _16384b,)>(); }

    #[test]
    fn mul_1() { let _: Val<_16384b> = val::<Mul, (_1b, _16384b,)>(); }

    #[test]
    fn mul() { let _: Val<_65536b> = val::<Mul, (_32b, _2048b,)>(); }
}
