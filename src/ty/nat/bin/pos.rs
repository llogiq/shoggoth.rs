use ty::bit::*;
use ty::{
    FnTm,
    Sig,
    Tm,
    Ty,
};

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

impl Tm<Pos> for _1 {}
impl<P: Tm<Pos>, B: Tm<Bit>> Tm<Pos> for (P, B) {}

/// Successor for type-level positive binary natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Succ {}
impl Sig for Succ { type Dom = Pos; type Cod = Pos; }
// 1 => 1:0
impl FnTm<Succ> for _1
{
    type O = (_1, _0,);
}
// p:0 => p:1
impl<P: Tm<Pos>> FnTm<Succ> for (P, _0)
{
    type O = (P, _1);
}
// p:1 => succ(p):0
impl<P: Tm<Pos>, Rec: Tm<Pos>> FnTm<Succ> for (P, _1) where
    P: FnTm<Succ, O = Rec>,
{
    type O = (Rec, _0);
}

/// Addition for type-level positive binary natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Add {}
impl Sig for Add { type Dom = (Pos, Pos); type Cod = Pos; }
// 1, 1 => 1:0
impl FnTm<Add> for ((_1), (_1))
{
    type O = (_1, _0);
}
// 1, q:0 => q:1
impl<P1: Tm<Pos>> FnTm<Add> for ((_1), (P1, _0))
{
    type O = (P1, _1);
}
// 1, q:1 => succ(q):0
impl<P1: Tm<Pos>, Rec: Tm<Pos>> FnTm<Add> for ((_1), (P1, _1)) where
    P1: FnTm<Succ, O = Rec>,
{
    type O = (Rec, _0);
}
// p:0, 1 => p:1
impl<P0: Tm<Pos>> FnTm<Add> for ((P0, _0), (_1))
{
    type O = (P0, _1);
}
// p:0, q:0 => (p + q):0
impl<P0: Tm<Pos>, P1: Tm<Pos>, Rec: Tm<Pos>> FnTm<Add> for ((P0, _0), (P1, _0)) where
    ((P0), (P1)): FnTm<Add, O = Rec>,
{
    type O = (Rec, _0);
}
// p:0, q:1 => (p + q):1
impl<P0: Tm<Pos>, P1: Tm<Pos>, Rec: Tm<Pos>> FnTm<Add> for ((P0, _0), (P1, _1)) where
    ((P0), (P1)): FnTm<Add, O = Rec>,
{
    type O = (Rec, _1);
}
// p:1, 1 => succ(p):0
impl<P0: Tm<Pos>, Rec: Tm<Pos>> FnTm<Add> for ((P0, _1), (_1)) where
    P0: FnTm<Succ, O = Rec>,
{
    type O = (Rec, _0);
}
// p:1, q:0 => (p + q):1
impl<P0: Tm<Pos>, P1: Tm<Pos>, Rec: Tm<Pos>> FnTm<Add> for ((P0, _1), (P1, _0,)) where
    ((P0), (P1),): FnTm<Add, O = Rec>,
{
    type O = (Rec, _1);
}
// p:1, q:1 => (p +carry q):1
impl<P0: Tm<Pos>, P1: Tm<Pos>, Rec: Tm<Pos>> FnTm<Add> for ((P0, _1), (P1, _1)) where
    ((P0), (P1),): FnTm<AddCarry, O = Rec>,
{
    type O = (Rec, _0);
}

// Addition with carry for type-level positive binary natural numbers
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
impl Sig for AddCarry { type Dom = (Pos, Pos); type Cod = Pos; }
// 1, 1 => 1:1
#[doc(hidden)]
impl FnTm<AddCarry> for ((_1), (_1),)
{
    type O = (_1, _1);
}
// 1, q:0 => succ(q):0
#[doc(hidden)]
impl<P1: Tm<Pos>, Rec: Tm<Pos>> FnTm<AddCarry> for ((_1), (P1, _0)) where
    P1: FnTm<Succ, O = Rec>,
{
    type O = (Rec, _0);
}
// 1, q:1 => succ(q):1
#[doc(hidden)]
impl<P1: Tm<Pos>, Rec: Tm<Pos>> FnTm<AddCarry> for ((_1), (P1, _1)) where
    P1: FnTm<Succ, O = Rec>,
{
    type O = (Rec, _1);
}
// p:0, 1 => p:1
#[doc(hidden)]
impl<P0: Tm<Pos>, Rec: Tm<Pos>> FnTm<AddCarry> for ((P0, _0), (_1)) where
    P0: FnTm<Succ, O = Rec>,
{
    type O = (P0, _0);
}
// p:0, q:0 => (p + q):1
#[doc(hidden)]
impl<P0: Tm<Pos>, P1: Tm<Pos>, Rec: Tm<Pos>> FnTm<AddCarry> for ((P0, _0), (P1, _0)) where
    ((P0), (P1)): FnTm<Add, O = Rec>,
{
    type O = (Rec, _1);
}
// p:0, q:1 => (p +carry q):0
#[doc(hidden)]
impl<P0: Tm<Pos>, P1: Tm<Pos>, Rec: Tm<Pos>> FnTm<AddCarry> for ((P0, _0), (P1, _1)) where
    ((P0), (P1)): FnTm<AddCarry, O = Rec>,
{
    type O = (Rec, _0);
}
// p:1, 1 => succ(p):1
#[doc(hidden)]
impl<P0: Tm<Pos>, Rec: Tm<Pos>> FnTm<AddCarry> for ((P0, _1), (_1)) where
    P0: FnTm<Succ, O = Rec>,
{
    type O = (Rec, _1);
}
// p:1, q:0 => (p +carry q):0
#[doc(hidden)]
impl<P0: Tm<Pos>, P1: Tm<Pos>, Rec: Tm<Pos>> FnTm<AddCarry> for ((P0, _1), (P1, _0)) where
    ((P0), (P1)): FnTm<AddCarry, O = Rec>,
{
    type O = (Rec, _0);
}
// p:1, q:1 => (p +carry q):1
#[doc(hidden)]
impl<P0: Tm<Pos>, P1: Tm<Pos>, Rec: Tm<Pos>> FnTm<AddCarry> for ((P0, _1), (P1, _1)) where
    ((P0), (P1)): FnTm<AddCarry, O = Rec>,
{
    type O = (Rec, _1);
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum PredDouble {}
impl Sig for PredDouble { type Dom = Pos; type Cod = Pos; }
// 1 => 1
impl FnTm<PredDouble> for _1
{
    type O = _1;
}
// p:0 => pred_double(p):1
impl<P: Tm<Pos>, Rec: Tm<Pos>> FnTm<PredDouble> for (P, _0) where
    P: FnTm<PredDouble, O = Rec>,
{
    type O = (Rec, _1);
}
// p:1 => p:0:1
impl<P: Tm<Pos>> FnTm<PredDouble> for (P, _1)
{
    type O = ((P, _0), _1);
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Pred {}
impl Sig for Pred { type Dom = Pos; type Cod = Pos; }
// p:1 => p:0
impl FnTm<Pred> for _1
{
    type O = _1;
}
// p:0 => pred_double(p)
impl<P: Tm<Pos>, Rec: Tm<Pos>> FnTm<Pred> for (P, _0,) where
    P: FnTm<PredDouble, O = Rec>,
{
    type O = Rec;
}
// p:1 => p:0
impl<P: Tm<Pos>> FnTm<Pred> for (P, _1)
{
    type O = (P, _0);
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
impl Sig for Mul { type Dom = (Pos, Pos); type Cod = Pos; }
// 1, q => q
impl<P1: Tm<Pos>> FnTm<Mul> for ((_1), (P1))
{
    type O = P1;
}
// p:0, q => (p * q):0
impl<P0: Tm<Pos>, P1: Tm<Pos>, Rec: Tm<Pos>> FnTm<Mul> for ((P0, _0), (P1)) where
    ((P0), (P1)): FnTm<Mul, O = Rec>,
{
    type O = (Rec, _0);
}
// p:1, q => (q + (p * q)):0
impl<P0: Tm<Pos>, P1: Tm<Pos>, Rec0: Tm<Pos>, Rec1: Tm<Pos>> FnTm<Mul> for ((P0, _1), (P1)) where
    ((P0), (P1)): FnTm<Mul, O = Rec0>,
    ((P1), (Rec0)): FnTm<Add, O = Rec1>,
{
    type O = Rec1;
}
