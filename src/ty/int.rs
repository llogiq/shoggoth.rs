use hlist::*;
use ty::{
    Ar1,
    Eval,
    Eval1,
    Infer,
    Tm,
    Ty,
    infer
};
use ty::bit::{
    _0,
    _1,
};
use ty::nat::pos;

/// Type-level integers
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Int {}

/// ```ignore
/// ---------
/// Int :: Ty
/// ```
impl Ty for Int {}

/// Type-level negative integers
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Zn<P: Tm<pos::Pos>> {}

/// Type-level positive integers
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Zp<P: Tm<pos::Pos>> {}

/// `0 : Int`
impl Tm<Int> for _0 {}

/// ```ignore
/// p : Pos
/// --------
/// -p : Int
/// ```
impl<N: Tm<pos::Pos>> Tm<Int> for Zn<N> {}

/// ```ignore
/// p : Pos
/// --------
/// +p : Int
/// ```
impl<N: Tm<pos::Pos>> Tm<Int> for Zp<N> {}

/// Type-level doubling for binary integers:
/// ```ignore
/// λx : Int. 2 * x
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Double {}

/// ```ignore
/// i : Int
/// ---------------
/// double(i) : Int
/// ```
impl Infer for Double {
    type Mode = infer::mode::Constant;
    type Ty = Ar1<Int, Int>;
}

/// `double(0) ==> 0`
impl Eval<Double> for HC<_0, HN> {
    type Out = _0;
}

/// `double(-p) ==> -(p:0)`
impl<P: Tm<pos::Pos>> Eval<Double> for HC<Zn<P>, HN> {
    type Out = Zn<(P, _0)>;
}

/// `double(+p) ==> +(p:0)`
impl<P: Tm<pos::Pos>> Eval<Double> for HC<Zp<P>, HN> {
    type Out = Zp<(P, _0)>;
}

/// Type-level doubling with successor for binary integers:
/// ```ignore
/// λx : Int. 2 * x + 1
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SuccDouble {}

/// ```ignore
/// i : Int
/// --------------------
/// succ_double(i) : Int
/// ```
impl Infer for SuccDouble {
    type Mode = infer::mode::Constant;
    type Ty = Ar1<Int, Int>;
}

/// `succ_double(0) ==> 1`
impl Eval<SuccDouble> for HC<_0, HN> {
    type Out = Zp<_1>;
}

/// `succ_double[Int](-p) ==> -(pred_double[Pos](p))`
impl<
       P: Tm<pos::Pos>,
     Rec: Tm<pos::Pos>,
> Eval<SuccDouble> for HC<Zn<P>, HN> where
       P: Eval1<pos::PredDouble, Out = Rec>,
{
    type Out = Zn<Rec>;
}

/// `succ_double(+p) ==> +(p:1)`
impl<P: Tm<pos::Pos>> Eval<SuccDouble> for HC<Zp<P>, HN>
{
    type Out = Zp<(P, _1)>;
}

/// Type-level doubling with predecessor for binary integers:
/// ```ignore
/// λx : Int. 2 * x - 1
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PredDouble {}

/// ```ignore
/// i : Int
/// --------------------
/// pred_double(i) : Int
/// ```
impl Infer for PredDouble {
    type Mode = infer::mode::Constant;
    type Ty = Ar1<Int, Int>;
}

/// `pred_double(0) ==> -1`
impl Eval<PredDouble> for HC<_0, HN> {
    type Out = Zn<_1>;
}

/// `pred_double(-p) ==> -(p:1)`
impl<P: Tm<pos::Pos>> Eval<PredDouble> for HC<Zn<P>, HN> {
    type Out = Zn<(P, _1)>;
}

/// `pred_double<Int>(+p) ==> +(pred_double<Pos>(p))`
impl<
       P: Tm<pos::Pos>,
     Rec: Tm<pos::Pos>,
> Eval<PredDouble> for HC<Zp<P>, HN> where
       P: Eval1<pos::PredDouble, Out = Rec>,
{
    type Out = Zp<Rec>;
}
