use hlist::*;
use ty::{
    Eval,
    Eval1,
    Infer,
    infer,
};
use ty::bit::*;

/// Type-level positive natural numbers (binary)
pub trait Pos {}
impl Pos for _1 {}
impl<P: Pos, B> Pos for (P, B) {}

/// Type-level successor for positive natural numbers
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Succ {}

impl Infer for Succ {
    type Arity = HC<(), HN>;
    type Mode = infer::mode::Constant;
}

/// `succ(1) ==> 1:0`
impl Eval<Succ> for HC<_1, HN> {
    type Out = (_1, _0,);
}

/// `succ(p:0) ==> p:1`
impl<P> Eval<Succ> for HC<(P, _0), HN> {
    type Out = (P, _1);
}

/// `p:1 ==> succ(p):0`
impl<P, Rec> Eval<Succ> for HC<(P, _1), HN> where
       P: Eval1<Succ, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// Type-level addition for positive natural numbers
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Add {}

impl Infer for Add {
    type Arity = HC<(), HC<(), HN>>;
    type Mode = infer::mode::Constant;
}

/// `add(1, 1) ==> 1:0`
impl Eval<Add> for HC<_1, HC<_1, HN>> {
    type Out = (_1, _0);
}

/// `add(1, q:0) ==> q:1`
impl<P1> Eval<Add> for HC<_1, HC<(P1, _0), HN>> {
    type Out = (P1, _1);
}

/// `add(1, q:1) ==> succ(q):0`
impl<P1, Rec> Eval<Add> for HC<_1, HC<(P1, _1), HN>> where
      P1: Eval1<Succ, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `add(p:0, 1) ==> p:1`
impl<P0> Eval<Add> for HC<(P0, _0), HC<_1, HN>> {
    type Out = (P0, _1);
}

/// `add(p:0, q:0) ==> add(p, q):0`
impl<P0, P1, Rec> Eval<Add> for HC<(P0, _0), HC<(P1, _0), HN>> where
    HC<P0, HC<P1, HN>>
        : Eval<Add, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `add(p:0, q:1) ==> add(p, q):1`
impl<P0, P1, Rec> Eval<Add> for HC<(P0, _0), HC<(P1, _1), HN>> where
    HC<P0, HC<P1, HN>>
        : Eval<Add, Out = Rec>,
{
    type Out = (Rec, _1);
}

/// `add(p:1, 1) ==> succ(p):0`
impl<P0, Rec> Eval<Add> for HC<(P0, _1), HC<_1, HN>> where
      P0: Eval1<Succ, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `add(p:1, q:0) ==> add(p, q):1`
impl<P0, P1, Rec> Eval<Add> for HC<(P0, _1), HC<(P1, _0), HN>> where
    HC<P0, HC<P1, HN>>
        : Eval<Add, Out = Rec>,
{
    type Out = (Rec, _1);
}

/// `add(p:1, q:1) ==> add_carry(p, q):1`
impl<P0, P1, Rec> Eval<Add> for HC<(P0, _1), HC<(P1, _1), HN>> where
    HC<P0, HC<P1, HN>>
        : Eval<AddCarry, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// Type-level addition with carry for positive natural numbers
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AddCarry {}

/// ```ignore
/// p : Pos
/// q : Pos
/// ---------------------
/// add_carry(p, q) : Pos
/// ```
impl Infer for AddCarry {
    type Arity = HC<(), HC<(), HN>>;
    type Mode = infer::mode::Constant;
}

/// `add_carry(1, 1) ==> 1:1`
impl Eval<AddCarry> for HC<_1, HC<_1, HN>> {
    type Out = (_1, _1);
}

/// `add_carry(1, q:0) ==> succ(q):0`
impl<P1, Rec> Eval<AddCarry> for HC<_1, HC<(P1, _0), HN>> where
      P1: Eval1<Succ, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `add_carry(1, q:1) ==> succ(q):1`
impl<P1, Rec> Eval<AddCarry> for HC<_1, HC<(P1, _1), HN>> where
      P1: Eval1<Succ, Out = Rec>,
{
    type Out = (Rec, _1);
}

/// `add_carry(p:0, 1) ==> p:1`
impl<P0, Rec> Eval<AddCarry> for HC<(P0, _0), HC<_1, HN>> where
      P0: Eval1<Succ, Out = Rec>,
{
    type Out = (P0, _0);
}

/// `add_carry(p:0, q:0) ==> add(p, q):1`
impl<P0, P1, Rec> Eval<AddCarry> for HC<(P0, _0), HC<(P1, _0), HN>> where
    HC<P0, HC<P1, HN>>
        : Eval<Add, Out = Rec>,
{
    type Out = (Rec, _1);
}

/// `add_carry(p:0, q:1) ==> add_carry(p, q):0`
impl<P0, P1, Rec> Eval<AddCarry> for HC<(P0, _0), HC<(P1, _1), HN>> where
    HC<P0, HC<P1, HN>>
        : Eval<AddCarry, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `add_carry(p:1, 1) ==> succ(p):1`
impl<P0, Rec> Eval<AddCarry> for HC<(P0, _1), HC<_1, HN>> where
      P0: Eval1<Succ, Out = Rec>,
{
    type Out = (Rec, _1);
}

/// `add_carry(p:1, q:0) ==> add_carry(p, q):0`
impl<P0, P1, Rec> Eval<AddCarry> for HC<(P0, _1), HC<(P1, _0), HN>> where
    HC<P0, HC<P1, HN>>
        : Eval<AddCarry, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `add_carry(p:1, q:1) ==> add_carry(p, q):1`
impl<P0, P1, Rec> Eval<AddCarry> for HC<(P0, _1), HC<(P1, _1), HN>> where
    HC<P0, HC<P1, HN>>
        : Eval<AddCarry, Out = Rec>,
{
    type Out = (Rec, _1);
}

/// `λx. 2 * x - 1`
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PredDouble {}

impl Infer for PredDouble {
    type Arity = HC<(), HN>;
    type Mode = infer::mode::Constant;
}

/// `pred_double(1) ==> 1`
impl Eval<PredDouble> for HC<_1, HN> {
    type Out = _1;
}

/// `pred_double(p:0) ==> pred_double(p):1`
impl<P, Rec> Eval<PredDouble> for HC<(P, _0), HN> where
       P: Eval1<PredDouble, Out = Rec>,
{
    type Out = (Rec, _1);
}

/// `pred_double(p:1) ==> p:0:1`
impl<P> Eval<PredDouble> for HC<(P, _1), HN> {
    type Out = ((P, _0), _1);
}

/// Type-level predecessor for positive natural numbers
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Pred {}

/// ```ignore
/// p : Pos
/// -------------
/// pred(p) : Pos
/// ```
impl Infer for Pred {
    type Arity = HC<(), HN>;
    type Mode = infer::mode::Constant;
}

/// `pred(p:1) ==> p:0`
impl Eval<Pred> for HC<_1, HN> {
    type Out = _1;
}

/// `p:0 ==> pred_double(p)`
impl<P, Rec> Eval<Pred> for HC<(P, _0), HN> where
       P: Eval1<PredDouble, Out = Rec>,
{
    type Out = Rec;
}

/// `p:1 ==> p:0`
impl<P> Eval<Pred> for HC<(P, _1), HN> {
    type Out = (P, _0);
}

/// Type-level multiplication for positive natural numbers
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Mul {}

impl Infer for Mul {
    type Arity = HC<(), HC<(), HN>>;
    type Mode = infer::mode::Constant;
}

/// `mul(1, q) ==> q`
impl<P1> Eval<Mul> for HC<_1, HC<P1, HN>> {
    type Out = P1;
}

/// `mul(p:0, q) ==> mul(p, q):0`
impl<P0, P1, Rec> Eval<Mul> for HC<(P0, _0), HC<P1, HN>> where
    HC<P0, HC<P1, HN>>
        : Eval<Mul, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `mul(p:1, q) ==> add(q, mul(p, q)):0`
impl<P0, P1, Rec0, Rec1> Eval<Mul> for HC<(P0, _1), HC<P1, HN>> where
    // mul(p0, p1) ==> r0
    HC<P0, HC<P1, HN>>
        : Eval<Mul, Out = Rec0>,
    // mul(p1, r0) ==> r1
    HC<P1, HC<Rec0, HN>>
        : Eval<Add, Out = Rec1>,
{
    type Out = Rec1;
}

#[cfg(test)]
mod test {
    use super::*;
    use ty::*;

    #[test]
    fn pred() {
        let x0 = Witness::<Ap1<nat::pos::Pred, (_1, _0)>>;
        let x1 = Witness::<_1>;
        x0 == x1;
    }
}
