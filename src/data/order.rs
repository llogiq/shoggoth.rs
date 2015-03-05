use std::marker::*;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Eq;
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GT;
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LT;

pub trait Comparison: MarkerTrait {}
impl Comparison for Eq {}
impl Comparison for GT {}
impl Comparison for LT {}

mod reflection {
    use data::reflect::*;
    use std;
    use super::*;

    impl Reflects for std::cmp::Ordering {
        fn reify(&self) -> Box<Reifies<Output = Self>> {
            match self {
                &std::cmp::Ordering::Equal   => { box Eq }
                &std::cmp::Ordering::Greater => { box GT }
                &std::cmp::Ordering::Less    => { box LT }
            }
        }
    }

    impl Reifies for Eq {
        type Output = std::cmp::Ordering;
        fn reflect(&self) -> std::cmp::Ordering {
            std::cmp::Ordering::Equal
        }
    }
    impl Reifies for GT {
        type Output = std::cmp::Ordering;
        fn reflect(&self) -> std::cmp::Ordering {
            std::cmp::Ordering::Greater
        }
    }
    impl Reifies for LT {
        type Output = std::cmp::Ordering;
        fn reflect(&self) -> std::cmp::Ordering {
            std::cmp::Ordering::Less
        }
    }
}
