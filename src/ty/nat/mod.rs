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
pub enum Nat {}

/// ```
/// ---------
/// Nat :: Ty
/// ```
impl Ty for Nat {}

/// ```
/// -------
/// 0 : Nat
/// ```
impl Tm<Nat> for _0 {}

/// ```
/// p : Pos
/// -------
/// p : Nat
/// ```
impl<P: Tm<Pos>> Tm<Nat> for P {}



/// Type-level successor for natural numbers
pub enum Succ {}

/// ```
/// n : Nat
/// -------------
/// succ(n) : Nat
/// ```
impl Sig for Succ { type Dom = Nat; type Cod = Nat; }

/// `succ(0) => 1`
impl Rule<Succ> for _0
{
    type O = _1;
}

/// `succ[nat](p) => succ[pos](p)`
impl<Rec, P> Rule<Succ> for P where
    Rec: Tm<Nat>,
    P: Rule<pos::Succ, O = Rec>,
{
    type O = Rec;
}



/// Type-level addition for natural numbers
pub enum Add {}

/// ```
/// m : Nat
/// n : Nat
/// -------------
/// add(m, n) : Nat
/// ```
impl Sig for Add { type Dom = HC<Nat, HC<Nat, HN>>; type Cod = Nat; }

/// `add(0, n) => n`
impl<P1> Rule<Add> for HC<_0, HC<P1, HN>> where
    P1: Tm<Pos>,
{
    type O = P1;
}

/// `add(m, 0) => m`
impl<P0> Rule<Add> for HC<P0, HC<_0, HN>> where
    P0: Tm<Pos>,
{
    type O = P0;
}

/// `add[nat](p, q) => add[pos](p, q)`
impl<P0, P1, Rec> Rule<Add> for HC<P0, HC<P1, HN>> where
    P0: Tm<Pos>,
    P1: Tm<Pos>,
    Rec: Tm<Nat>,
    HC<P0, HC<P1, HN>>: Rule<pos::Add, O = Rec>,
{
    type O = Rec;
}



/// Type-level multiplication for natural numbers
pub enum Mul {}

/// ```
/// m : Nat
/// n : Nat
/// -------------
/// mul(m, n) : Nat
/// ```
impl Sig for Mul { type Dom = HC<Nat, HC<Nat, HN>>; type Cod = Nat; }

/// `mul(0, n) => 0`
impl<P1> Rule<Mul> for HC<_0, HC<P1, HN>> where
    P1: Tm<Pos>,
{
    type O = _0;
}

/// `mul(m, 0) => 0`
impl<P0> Rule<Mul> for HC<P0, HC<_0, HN>> where
    P0: Tm<Pos>,
{
    type O = _0;
}

/// `mul[nat](p, q) => mul[pos](p, q)`
impl<P0, P1, Rec> Rule<Mul> for HC<P0, HC<P1, HN>> where
    P0: Tm<Pos>,
    P1: Tm<Pos>,
    Rec: Tm<Nat>,
    HC<P0, HC<P1, HN>>: Rule<pos::Mul, O = Rec>,
{
    type O = Rec;
}

#[cfg(test)]
mod test {
    use hlist::*;
    use super::*;
    use ty::*;

    // FIXME: add algebraic tests

    #[test]
    fn add_0() {
        let x: Wit<HC<_0b, HC<_16384b, HN>>> = Wit;
        let _: Wit<_16384b> = x.app::<Add>();
    }

    #[test]
    fn add() {
        let x: Wit<HC<_8192b, HC<_8192b, HN>>> = Wit;
        let _: Wit<_16384b> = x.app::<Add>();
    }

    #[test]
    fn mul_0() {
        let x: Wit<HC<_0b, HC<_16384b, HN>>> = Wit;
        let _: Wit<_0b> = x.app::<Mul>();
    }

    #[test]
    fn mul_1() {
        let x: Wit<HC<_1b, HC<_16384b, HN>>> = Wit;
        let _: Wit<_16384b> = x.app::<Mul>();
    }

    #[test]
    fn mul() {
        let x: Wit<HC<_32b, HC<_2048b, HN>>> = Wit;
        let _: Wit<_65536b> = x.app::<Mul>();
    }
}
