use hlist::{
    HC,
    HN,
};
use self::pos::{
    Pos,
};
use ty::{
    FnTm,
    Sig,
    Tm,
    Ty,
};
pub use ty::bit::{
    _0,
    _1,
};

/// Binary positive natural numbers
pub mod pos;

/// Binary natural numbers
pub enum Nat {}
impl Ty for Nat {}

impl Tm<Nat> for _0 {}
impl<P: Tm<Pos>> Tm<Nat> for P {}

/// Type-level successor for binary natural numbers
pub enum Succ {}
impl Sig for Succ { type Dom = Nat; type Cod = Nat; }
// 0 => 1
impl FnTm<Succ> for _0
{
    type O = _1;
}
// p => succ(p)
impl<P: Tm<Pos>, Rec: Tm<Nat>> FnTm<Succ> for P where
    P: FnTm<pos::Succ, O = Rec>,
{
    type O = Rec;
}

/// Type-level addition for binary natural numbers
pub enum Add {}
impl Sig for Add { type Dom = HC<Nat, HC<Nat, HN>>; type Cod = Nat; }
impl<P1: Tm<Pos>> FnTm<Add> for HC<_0, HC<P1, HN>>
// 0, n => n
{
    type O = P1;
}
impl<P0: Tm<Pos>> FnTm<Add> for HC<P0, HC<_0, HN>>
// m, 0 => m
{
    type O = P0;
}
// p, q => p + q
impl<P0: Tm<Pos>, P1: Tm<Pos>, Rec: Tm<Nat>> FnTm<Add> for HC<P0, HC<P1, HN>> where
    HC<P0, HC<P1, HN>>: FnTm<pos::Add, O = Rec>,
{
    type O = Rec;
}

/// Type-level multiplication for binary natural numbers
pub enum Mul {}
impl Sig for Mul { type Dom = HC<Nat, HC<Nat, HN>>; type Cod = Nat; }
// 0, n => 0
impl<P1: Tm<Pos>> FnTm<Mul> for HC<_0, HC<P1, HN>>
{
    type O = _0;
}
// m, 0 => 0
impl<P0: Tm<Pos>> FnTm<Mul> for HC<P0, HC<_0, HN>>
{
    type O = _0;
}
// p, q => p * q
impl<P0: Tm<Pos>, P1: Tm<Pos>, Rec: Tm<Nat>> FnTm<Mul> for HC<P0, HC<P1, HN>> where
    HC<P0, HC<P1, HN>>: FnTm<pos::Mul, O = Rec>,
{
    type O = Rec;
}

#[cfg(test)]
mod tests {
    use super::*;
    use ty::literal::*;
    use ty::wit::*;

    // FIXME: add algebraic tests

    #[test]
    fn add_0() { let _: Wit<_16384b> = wit::<Add, HC<_0b, HC<_16384b, HN>>>(); }

    #[test]
    fn add() { let _: Wit<_16384b> = wit::<Add, HC<_8192b, HC<_8192b, HN>>>(); }

    #[test]
    fn mul_0() { let _: Wit<_0b> = wit::<Mul, HC<_0b, HC<_16384b, HN>>>(); }

    #[test]
    fn mul_1() { let _: Wit<_16384b> = wit::<Mul, HC<_1b, HC<_16384b, HN>>>(); }

    #[test]
    fn mul() { let _: Wit<_65536b> = wit::<Mul, HC<_32b, HC<_2048b, HN>>>(); }
}
