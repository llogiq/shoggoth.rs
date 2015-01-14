use ty::{
    Tm,
    Ty,
    fun,
};

/// Type-level bits
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Bit {}
impl Ty for Bit {}

/// Type-level bit `0`
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum _0b {}
impl Tm<Bit> for _0b {}

/// Type-level bit `1`
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum _1b {}
impl Tm<Bit> for _1b {}

/// Positive binary natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Pos {}
impl Ty for Pos {}

impl Tm<Pos> for _1b {}
impl<P: Tm<Pos>, B: Tm<Bit>> Tm<Pos> for (P, B,) {}

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

impl Tm<Nat> for _0b {}
impl<P: Tm<Pos>> Tm<Nat> for P {}

/// Type-level successor for positive binary natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Succ {}
impl fun::Sig for Succ { type Dom = (Pos,); type Cod = Pos; }
// 1 => 1:0
impl fun::Fn<Succ> for (_1b,)
{
    type O = (_1b, _0b,);
}
// p:0 => p:1
impl<P: Tm<Pos>> fun::Fn<Succ> for ((P, _0b,),)
{
    type O = (P, _1b,);
}
// p:1 => s(p):0
impl<P: Tm<Pos>, Rec: Tm<Pos>> fun::Fn<Succ> for ((P, _1b,),) where
    (P,): fun::Fn<Succ, O = Rec>,
{
    type O = (Rec, _0b,);
}

/// Type-level addition for positive binary natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Add {}
impl fun::Sig for Add { type Dom = (Pos, Pos,); type Cod = Pos; }
// 1, 1 => 1:0
impl fun::Fn<Add> for ((_1b), (_1b),)
{
    type O = (_1b, _0b,);
}
// 1, q:0 => q:1
impl<Q: Tm<Pos>> fun::Fn<Add> for ((_1b), (Q, _0b,),)
{
    type O = (Q, _1b,);
}
// 1, q:1 => s(q):0
impl<Q: Tm<Pos>, Rec: Tm<Pos>> fun::Fn<Add> for ((_1b), (Q, _1b,),) where
    (Q,): fun::Fn<Succ, O = Rec>,
{
    type O = (Rec, _0b,);
}
// p:0, 1 => p:1
impl<P: Tm<Pos>> fun::Fn<Add> for ((P, _0b,), (_1b),)
{
    type O = (P, _1b,);
}
// p:0, q:0 => (p + q):0
impl<P: Tm<Pos>, Q: Tm<Pos>, Rec: Tm<Pos>> fun::Fn<Add> for ((P, _0b,), (Q, _0b),) where
    (P, Q,): fun::Fn<Add, O = Rec>,
{
    type O = (Rec, _0b,);
}
// p:0, q:1 => (p + q):1
impl<P: Tm<Pos>, Q: Tm<Pos>, Rec: Tm<Pos>> fun::Fn<Add> for ((P, _0b,), (Q, _1b),) where
    (P, Q,): fun::Fn<Add, O = Rec>,
{
    type O = (Rec, _1b,);
}
// p:1, 1 => s(p):0
impl<P: Tm<Pos>, Rec: Tm<Pos>> fun::Fn<Add> for ((P, _1b,), (_1b),) where
    (P,): fun::Fn<Succ, O = Rec>,
{
    type O = (Rec, _0b,);
}
// p:1, q:0 => (p + q):1
impl<P: Tm<Pos>, Q: Tm<Pos>, Rec: Tm<Pos>> fun::Fn<Add> for ((P, _1b,), (Q, _0b,),) where
    (P, Q,): fun::Fn<Add, O = Rec>,
{
    type O = (Rec, _1b,);
}
// p:1, q:1 => (p +carry q):1
impl<P: Tm<Pos>, Q: Tm<Pos>, Rec: Tm<Pos>> fun::Fn<Add> for ((P, _1b,), (Q, _1b,),) where
    (P, Q,): fun::Fn<AddCarry, O = Rec>,
{
    type O = (Rec, _0b,);
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
#[doc(hidden)]
pub enum AddCarry {}
#[doc(hidden)]
impl fun::Sig for AddCarry { type Dom = (Pos, Pos,); type Cod = Pos; }
// 1, 1 => 1:1
#[doc(hidden)]
impl fun::Fn<AddCarry> for ((_1b), (_1b),)
{
    type O = (_1b, _1b,);
}
// 1, q:0 => s(q):0
#[doc(hidden)]
impl<Q: Tm<Pos>, Rec: Tm<Pos>> fun::Fn<AddCarry> for ((_1b), (Q, _0b,),) where
    (Q,): fun::Fn<Succ, O = Rec>,
{
    type O = (Rec, _0b,);
}
// 1, q:1 => s(q):1
#[doc(hidden)]
impl<Q: Tm<Pos>, Rec: Tm<Pos>> fun::Fn<AddCarry> for ((_1b), (Q, _1b,),) where
    (Q,): fun::Fn<Succ, O = Rec>,
{
    type O = (Rec, _1b,);
}
// p:0, 1 => p:1
#[doc(hidden)]
impl<P: Tm<Pos>, Rec: Tm<Pos>> fun::Fn<AddCarry> for ((P, _0b,), (_1b),) where
    (P,): fun::Fn<Succ, O = Rec>,
{
    type O = (P, _0b,);
}
// p:0, q:0 => (p + q):1
#[doc(hidden)]
impl<P: Tm<Pos>, Q: Tm<Pos>, Rec: Tm<Pos>> fun::Fn<AddCarry> for ((P, _0b,), (Q, _0b),) where
    (P, Q,): fun::Fn<Add, O = Rec>,
{
    type O = (Rec, _1b,);
}
// p:0, q:1 => (p +carry q):0
#[doc(hidden)]
impl<P: Tm<Pos>, Q: Tm<Pos>, Rec: Tm<Pos>> fun::Fn<AddCarry> for ((P, _0b,), (Q, _1b),) where
    (P, Q,): fun::Fn<AddCarry, O = Rec>,
{
    type O = (Rec, _0b,);
}
// p:1, 1 => s(p):1
#[doc(hidden)]
impl<P: Tm<Pos>, Rec: Tm<Pos>> fun::Fn<AddCarry> for ((P, _1b,), (_1b),) where
    (P,): fun::Fn<Succ, O = Rec>,
{
    type O = (Rec, _1b,);
}
// p:1, q:0 => (p +carry q):0
#[doc(hidden)]
impl<P: Tm<Pos>, Q: Tm<Pos>, Rec: Tm<Pos>> fun::Fn<AddCarry> for ((P, _1b,), (Q, _0b,),) where
    (P, Q,): fun::Fn<AddCarry, O = Rec>,
{
    type O = (Rec, _0b,);
}
// p:1, q:1 => (p +carry q):1
#[doc(hidden)]
impl<P: Tm<Pos>, Q: Tm<Pos>, Rec: Tm<Pos>> fun::Fn<AddCarry> for ((P, _1b,), (Q, _1b,),) where
    (P, Q,): fun::Fn<AddCarry, O = Rec>,
{
    type O = (Rec, _1b,);
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Mul {}
impl fun::Sig for Mul { type Dom = (Pos, Pos,); type Cod = Pos; }
impl<Q: Tm<Pos>> fun::Fn<Mul> for ((_1b), Q,)
{
    type O = Q;
}
impl<P: Tm<Pos>, Q: Tm<Pos>, Rec: Tm<Pos>> fun::Fn<Mul> for ((P, _0b,), Q,) where
    (P, Q,): fun::Fn<Mul, O = Rec>,
{
    type O = (Rec, _0b,);
}
impl<P: Tm<Pos>, Q: Tm<Pos>, Rec0: Tm<Pos>, Rec1: Tm<Pos>> fun::Fn<Mul> for ((P, _1b,), Q,) where
    (P, Q,): fun::Fn<Mul, O = Rec0>,
    (Q, Rec0,): fun::Fn<Add, O = Rec1>,
{
    type O = Rec1;
}

#[cfg(test)]
mod tests {
    use super::{
        Add,
        AddCarry,
        Mul,
    };
    use ty::literal::*;
    use ty::val::*;

    #[test]
    fn add() { let _: Val< _16384b > = val::<Add, (_8192b, _8192b,) >(); }

    #[test]
    fn mul() { let _: Val< _65536b > = val::<Mul, (_32b, _2048b,) >(); }
}
