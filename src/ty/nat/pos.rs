use hlist::{
    HCons,
    HNil,
};
use ty::bit::*;
use ty::{
    Rule,
    Sig,
    Tm,
    Ty,
};

/// Type-level positive natural numbers (binary)
pub enum Pos {}

/// ```ignore
/// ---------
/// Pos :: Ty
/// ```
impl Ty for Pos {}

/// ```ignore
/// -------
/// 1 : Pos
/// ```
impl Tm<Pos> for _1 {}

/// ```ignore
/// p : Pos
/// b : Bit
/// ------------
/// (p, b) : Pos
/// ```
impl<P, B> Tm<Pos> for (P, B) where
    P: Tm<Pos>,
    B: Tm<Bit>,
{}



/// Type-level successor for positive natural numbers
pub enum Succ {}

/// ```ignore
/// p : Pos
/// -------------
/// succ(p) : Pos
/// ```
impl Sig for Succ { type Dom = Pos; type Cod = Pos; }

/// `succ(1) => 1:0`
impl Rule<Succ> for _1
{
    type Out = (_1, _0,);
}

/// `succ(p:0) => p:1`
impl<P> Rule<Succ> for (P, _0) where
    P: Tm<Pos>,
{
    type Out = (P, _1);
}

/// `p:1 => succ(p):0`
impl<P, Rec> Rule<Succ> for (P, _1) where
    P: Tm<Pos>,
    Rec: Tm<Pos>,
    P: Rule<Succ, Out = Rec>,
{
    type Out = (Rec, _0);
}



/// Type-level addition for positive natural numbers
pub enum Add {}

/// ```ignore
/// p : Pos
/// q : Pos
/// ---------------
/// add(p, q) : Pos
/// ```
impl Sig for Add { type Dom = HCons<Pos, HCons<Pos, HNil>>; type Cod = Pos; }

/// `add(1, 1) => 1:0`
impl Rule<Add> for HCons<_1, HCons<_1, HNil>>
{
    type Out = (_1, _0);
}

/// `add(1, q:0) => q:1`
impl<P1: Tm<Pos>> Rule<Add> for HCons<_1, HCons<(P1, _0), HNil>>
{
    type Out = (P1, _1);
}

/// `add(1, q:1) => succ(q):0`
impl<P1, Rec> Rule<Add> for HCons<_1, HCons<(P1, _1), HNil>> where
    P1: Tm<Pos>,
    Rec: Tm<Pos>,
    P1: Rule<Succ, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `add(p:0, 1) => p:1`
impl<P0> Rule<Add> for HCons<(P0, _0), HCons<_1, HNil>> where
    P0: Tm<Pos>,
{
    type Out = (P0, _1);
}

/// `add(p:0, q:0) => add(p, q):0`
impl<P0, P1, Rec> Rule<Add> for HCons<(P0, _0), HCons<(P1, _0), HNil>> where
    P0: Tm<Pos>,
    P1: Tm<Pos>,
    Rec: Tm<Pos>,
    HCons<P0, HCons<P1, HNil>>: Rule<Add, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `add(p:0, q:1) => add(p, q):1`
impl<P0, P1, Rec> Rule<Add> for HCons<(P0, _0), HCons<(P1, _1), HNil>> where
    P0: Tm<Pos>,
    P1: Tm<Pos>,
    Rec: Tm<Pos>,
    HCons<P0, HCons<P1, HNil>>: Rule<Add, Out = Rec>,
{
    type Out = (Rec, _1);
}

/// `add(p:1, 1) => succ(p):0`
impl<P0, Rec> Rule<Add> for HCons<(P0, _1), HCons<_1, HNil>> where
    P0: Tm<Pos>,
    Rec: Tm<Pos>,
    P0: Rule<Succ, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `add(p:1, q:0) => add(p, q):1`
impl<P0, P1, Rec> Rule<Add> for HCons<(P0, _1), HCons<(P1, _0), HNil>> where
    P0: Tm<Pos>,
    P1: Tm<Pos>,
    Rec: Tm<Pos>,
    HCons<P0, HCons<P1, HNil>>: Rule<Add, Out = Rec>,
{
    type Out = (Rec, _1);
}

/// `add(p:1, q:1) => add_carry(p, q):1`
impl<P0, P1, Rec> Rule<Add> for HCons<(P0, _1), HCons<(P1, _1), HNil>> where
    P0: Tm<Pos>,
    P1: Tm<Pos>,
    Rec: Tm<Pos>,
    HCons<P0, HCons<P1, HNil>>: Rule<AddCarry, Out = Rec>,
{
    type Out = (Rec, _0);
}



/// Type-level addition with carry for positive natural numbers
pub enum AddCarry {}

/// ```ignore
/// p : Pos
/// q : Pos
/// ---------------------
/// add_carry(p, q) : Pos
/// ```
impl Sig for AddCarry { type Dom = HCons<Pos, HCons<Pos, HNil>>; type Cod = Pos; }

/// `add_carry(1, 1) => 1:1`
impl Rule<AddCarry> for HCons<_1, HCons<_1, HNil>>
{
    type Out = (_1, _1);
}

/// `add_carry(1, q:0) => succ(q):0`
impl<P1, Rec> Rule<AddCarry> for HCons<_1, HCons<(P1, _0), HNil>> where
    P1: Tm<Pos>,
    Rec: Tm<Pos>,
    P1: Rule<Succ, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `add_carry(1, q:1) => succ(q):1`
impl<P1, Rec> Rule<AddCarry> for HCons<_1, HCons<(P1, _1), HNil>> where
    P1: Tm<Pos>,
    Rec: Tm<Pos>,
    P1: Rule<Succ, Out = Rec>,
{
    type Out = (Rec, _1);
}

/// `add_carry(p:0, 1) => p:1`
impl<P0, Rec> Rule<AddCarry> for HCons<(P0, _0), HCons<_1, HNil>> where
    P0: Tm<Pos>,
    Rec: Tm<Pos>,
    P0: Rule<Succ, Out = Rec>,
{
    type Out = (P0, _0);
}

/// `add_carry(p:0, q:0) => add(p, q):1`
impl<P0, P1, Rec> Rule<AddCarry> for HCons<(P0, _0), HCons<(P1, _0), HNil>> where
    P0: Tm<Pos>,
    P1: Tm<Pos>,
    Rec: Tm<Pos>,
    HCons<P0, HCons<P1, HNil>>: Rule<Add, Out = Rec>,
{
    type Out = (Rec, _1);
}

/// `add_carry(p:0, q:1) => add_carry(p, q):0`
impl<P0, P1, Rec> Rule<AddCarry> for HCons<(P0, _0), HCons<(P1, _1), HNil>> where
    P0: Tm<Pos>,
    P1: Tm<Pos>,
    Rec: Tm<Pos>,
    HCons<P0, HCons<P1, HNil>>: Rule<AddCarry, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `add_carry(p:1, 1) => succ(p):1`
impl<P0: Tm<Pos>, Rec: Tm<Pos>> Rule<AddCarry> for HCons<(P0, _1), HCons<_1, HNil>> where
    P0: Rule<Succ, Out = Rec>,
{
    type Out = (Rec, _1);
}

/// `add_carry(p:1, q:0) => add_carry(p, q):0`
impl<P0, P1, Rec> Rule<AddCarry> for HCons<(P0, _1), HCons<(P1, _0), HNil>> where
    P0: Tm<Pos>,
    P1: Tm<Pos>,
    Rec: Tm<Pos>,
    HCons<P0, HCons<P1, HNil>>: Rule<AddCarry, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `add_carry(p:1, q:1) => add_carry(p, q):1`
impl<P0, P1, Rec> Rule<AddCarry> for HCons<(P0, _1), HCons<(P1, _1), HNil>> where
    P0: Tm<Pos>,
    P1: Tm<Pos>,
    Rec: Tm<Pos>,
    HCons<P0, HCons<P1, HNil>>: Rule<AddCarry, Out = Rec>,
{
    type Out = (Rec, _1);
}



pub enum PredDouble {}

/// ```ignore
/// p : Pos
/// -------------------
/// pred_carry(p) : Pos
/// ```
impl Sig for PredDouble { type Dom = Pos; type Cod = Pos; }

/// `pred_carry(1) => 1`
impl Rule<PredDouble> for _1
{
    type Out = _1;
}

/// `pred_carry(p:0) => pred_double(p):1`
impl<P, Rec> Rule<PredDouble> for (P, _0) where
    P: Tm<Pos>,
    Rec: Tm<Pos>,
    P: Rule<PredDouble, Out = Rec>,
{
    type Out = (Rec, _1);
}

/// `pred_carry(p:1) => p:0:1`
impl<P> Rule<PredDouble> for (P, _1) where
    P: Tm<Pos>,
{
    type Out = ((P, _0), _1);
}



/// Type-level predecessor for positive natural numbers
pub enum Pred {}

/// ```ignore
/// p : Pos
/// -------------
/// pred(p) : Pos
/// ```
impl Sig for Pred { type Dom = Pos; type Cod = Pos; }

/// `pred(p:1) => p:0`
impl Rule<Pred> for _1
{
    type Out = _1;
}

/// `p:0 => pred_double(p)`
impl<P, Rec> Rule<Pred> for (P, _0) where
    P: Tm<Pos>,
    Rec: Tm<Pos>,
    P: Rule<PredDouble, Out = Rec>,
{
    type Out = Rec;
}

/// `p:1 => p:0`
impl<P> Rule<Pred> for (P, _1) where
    P: Tm<Pos>,
{
    type Out = (P, _0);
}



/// Type-level multiplication for positive natural numbers
pub enum Mul {}

/// ```ignore
/// p : Pos
/// q : Pos
/// ---------------
/// mul(p, q) : Pos
/// ```
impl Sig for Mul { type Dom = HCons<Pos, HCons<Pos, HNil>>; type Cod = Pos; }

/// `mul(1, q) => q`
impl<P1> Rule<Mul> for HCons<_1, HCons<P1, HNil>> where
    P1: Tm<Pos>
{
    type Out = P1;
}

/// `mul(p:0, q) => mul(p, q):0`
impl<P0, P1, Rec> Rule<Mul> for HCons<(P0, _0), HCons<P1, HNil>> where
    P0: Tm<Pos>,
    P1: Tm<Pos>,
    Rec: Tm<Pos>,
    HCons<P0, HCons<P1, HNil>>: Rule<Mul, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `mul(p:1, q) => add(q, mul(p, q)):0`
impl<P0, P1, Rec0, Rec1> Rule<Mul> for HCons<(P0, _1), HCons<P1, HNil>> where
    P0: Tm<Pos>,
    P1: Tm<Pos>,
    Rec0: Tm<Pos>,
    Rec1: Tm<Pos>,
    HCons<P0, HCons<P1, HNil>>: Rule<Mul, Out = Rec0>,
    HCons<P1, HCons<Rec0, HNil>>: Rule<Add, Out = Rec1>,
{
    type Out = Rec1;
}
