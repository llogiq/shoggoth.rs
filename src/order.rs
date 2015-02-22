use std::marker::{
    PhantomFn,
};

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Eq;
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GT;
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LT;

pub trait Comparison: PhantomFn<Self> {}
impl Comparison for Eq {}
impl Comparison for GT {}
impl Comparison for LT {}
