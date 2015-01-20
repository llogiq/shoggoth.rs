use ty::{
    Rule,
    Sig,
    Tm,
    Ty,
};
use ty::nat::pos;
use ty::bit::{
    _0,
    _1,
};

/// Type-level integers
pub enum Int {}

/// ```
/// ---------
/// Int :: Ty
/// ```
impl Ty for Int {}

/// Type-level negative integers
pub enum Zn<P> where
    P: Tm<pos::Pos>,
{}

/// Type-level positive integers
pub enum Zp<P> where
    P: Tm<pos::Pos>,
{}

/// `0 : Int`
impl Tm<Int> for _0 {}

/// ```
/// p : Pos
/// --------
/// -p : Int
/// ```
impl<N> Tm<Int> for Zn<N> where
    N: Tm<pos::Pos>,
{}

/// ```
/// p : Pos
/// --------
/// +p : Int
/// ```
impl<N> Tm<Int> for Zp<N> where
    N: Tm<pos::Pos>,
{}



/// Type-level doubling for binary integers:
/// ```
/// λx:int. 2 * x
/// ```
pub enum Double {}

/// ```
/// i : Int
/// ---------------
/// double(i) : Int
/// ```
impl Sig for Double {
    type Dom = Int;
    type Cod = Int;
}

/// `double(0) => 0`
impl Rule<Double> for _0
{
    type O = _0;
}

/// `double(-p) => -(p:0)`
impl<P> Rule<Double> for Zn<P> where
    P: Tm<pos::Pos>,
{
    type O = Zn<(P, _0)>;
}

/// `double(+p) => +(p:0)`
impl<P> Rule<Double> for Zp<P> where
    P: Tm<pos::Pos>,
{
    type O = Zp<(P, _0)>;
}



/// Type-level doubling with successor for binary integers:
/// ```
/// λx:int. 2 * x + 1
/// ```
pub enum SuccDouble {}

/// ```
/// i : Int
/// --------------------
/// succ_double(i) : Int
/// ```
impl Sig for SuccDouble { type Dom = Int; type Cod = Int; }

/// `succ_double(0) => 1`
impl Rule<SuccDouble> for _0
{
    type O = Zp<_1>;
}

/// `succ_double[int](-p) => -(pred_double[pos](p))`
impl<P, Rec> Rule<SuccDouble> for Zn<P> where
    P: Tm<pos::Pos>,
    Rec: Tm<pos::Pos>,
    P: Rule<pos::PredDouble, O = Rec>,
{
    type O = Zn<Rec>;
}

/// `succ_double(+p) => +(p:1)`
impl<P> Rule<SuccDouble> for Zp<P> where
    P: Tm<pos::Pos>,
{
    type O = Zp<(P, _1)>;
}



/// Type-level doubling with predecessor for binary integers:
/// ```
/// λx:int. 2 * x - 1
/// ```
pub enum PredDouble {}

/// ```
/// i : Int
/// --------------------
/// pred_double(i) : Int
/// ```
impl Sig for PredDouble {
    type Dom = Int;
    type Cod = Int;
}

/// `pred_double(0) => -1`
impl Rule<PredDouble> for _0
{
    type O = Zn<_1>;
}

/// `pred_double(-p) => -(p:1)`
impl<P> Rule<PredDouble> for Zn<P> where
    P: Tm<pos::Pos>,
{
    type O = Zn<(P, _1)>;
}

/// `pred_double[int](+p) => +(pred_double[pos](p))`
impl<P, Rec> Rule<PredDouble> for Zp<P> where
    P: Tm<pos::Pos>,
    Rec: Tm<pos::Pos>,
    P: Rule<pos::PredDouble, O = Rec>,
{
    type O = Zp<Rec>;
}
