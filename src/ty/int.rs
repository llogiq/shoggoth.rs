use ty::{
    Tm,
    Ty,
    fun,
};
use ty::nat::bin::pos as npos;
use ty::bit::{
    _0,
    _1,
};

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Int {}
impl Ty for Int {}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Nz<P: Tm<npos::Pos>> {}
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Pz<P: Tm<npos::Pos>> {}

impl Tm<Int> for _0 {}
impl<N: Tm<npos::Pos>> Tm<Int> for Nz<N> {}
impl<N: Tm<npos::Pos>> Tm<Int> for Pz<N> {}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Double {}
impl fun::Sig for Double { type Dom = (Int,); type Cod = Int; }
// 0 => 0
impl fun::Fn<Double> for ((_0),)
{
    type O = _0;
}
// -p => -p:0
impl<P: Tm<npos::Pos>> fun::Fn<Double> for (Nz<P>,)
{
    type O = Nz<(P, _0,)>;
}
// +p => +p:0
impl<P: Tm<npos::Pos>> fun::Fn<Double> for (Pz<P>,)
{
    type O = Pz<(P, _0,)>;
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum SuccDouble {}
impl fun::Sig for SuccDouble { type Dom = (Int,); type Cod = Int; }
// 0 => 0
impl fun::Fn<SuccDouble> for ((_0),)
{
    type O = Pz<_1>;
}
// -p => -p:0
impl<P: Tm<npos::Pos>, Rec: Tm<npos::Pos>> fun::Fn<SuccDouble> for (Nz<P>,) where
    (P,): fun::Fn<npos::Pos, O = Rec>,
{
    type O = Nz<Rec>;
}
// +p => +p:0
impl<P: Tm<npos::Pos>> fun::Fn<SuccDouble> for (Pz<P>,)
{
    type O = Pz<(P, _1,)>;
}
