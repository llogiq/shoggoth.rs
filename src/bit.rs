use std::marker::PhantomFn;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct _0;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct _1;

pub trait IsBit: PhantomFn<Self> {}
impl IsBit for _0 {}
impl IsBit for _1 {}
