use nat;
use std::marker::PhantomFn;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct _0;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct _1;

pub trait Bit: PhantomFn<Self> + nat::IsNat {}
impl Bit for _0 {}
impl Bit for _1 {}
