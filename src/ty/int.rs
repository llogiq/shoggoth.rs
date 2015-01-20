use ty::{
    Rule,
    Sig,
    Tm,
    Ty,
};
use ty::nat::pos as npos;
use ty::bit::{
    _0,
    _1,
};

pub enum Int {}

impl Ty for Int {}

pub enum Nz<P: Tm<npos::Pos>> {}
pub enum Pz<P: Tm<npos::Pos>> {}


impl Tm<Int> for _0 {}
impl<N: Tm<npos::Pos>> Tm<Int> for Nz<N> {}
impl<N: Tm<npos::Pos>> Tm<Int> for Pz<N> {}





pub enum Double {}
impl Sig for Double { type Dom = Int; type Cod = Int; }
// 0 => 0


impl Rule<Double> for _0
{
    type O = _0;
}
// -p => -p:0
impl<P: Tm<npos::Pos>> Rule<Double> for Nz<P>

{
    type O = Nz<(P, _0)>;
}
// +p => +p:0
impl<P: Tm<npos::Pos>> Rule<Double> for Pz<P>

{
    type O = Pz<(P, _0)>;
}



pub enum SuccDouble {}

impl Sig for SuccDouble { type Dom = Int; type Cod = Int; }
// 0 => 0

impl Rule<SuccDouble> for _0
{
    type O = Pz<_1>;
}
// -p => -p:0
impl<P: Tm<npos::Pos>, Rec: Tm<npos::Pos>> Rule<SuccDouble> for Nz<P> where
    P: Rule<npos::Pos, O = Rec>,

{
    type O = Nz<Rec>;
}
// +p => +p:0
impl<P: Tm<npos::Pos>> Rule<SuccDouble> for Pz<P>

{
    type O = Pz<(P, _1)>;
}



pub enum PredDouble {}
impl Sig for PredDouble { type Dom = Int; type Cod = Int; }
// 0 => 0


impl Rule<PredDouble> for _0
{
    type O = Nz<_1>;
}
// -p => -p:0
impl<P: Tm<npos::Pos>> Rule<PredDouble> for Nz<P>

{
    type O = Nz<(P, _1)>;
}
// +p => +p:0
impl<P: Tm<npos::Pos>, Rec: Tm<npos::Pos>> Rule<PredDouble> for Pz<P> where
    (P,): Rule<npos::Pos, O = Rec>,

{
    type O = Pz<Rec>;
}
