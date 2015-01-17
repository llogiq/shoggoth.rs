use ty::{
    Tm,
    Ty,
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
