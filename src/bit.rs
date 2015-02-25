use nat;
use std::marker::{
    MarkerTrait,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct _0;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct _1;

pub trait Bit: MarkerTrait + nat::IsNat {}
impl Bit for _0 {}
impl Bit for _1 {}
