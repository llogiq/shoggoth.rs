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

mod reflection {
    use reflect::Reifies;
    use std;
    use super::*;

    impl Reifies for Eq {
        type Output = std::cmp::Ordering;
        #[inline(always)]
        fn reflect(&self) -> std::cmp::Ordering {
            std::cmp::Ordering::Equal
        }
    }
    impl Reifies for GT {
        type Output = std::cmp::Ordering;
        #[inline(always)]
        fn reflect(&self) -> std::cmp::Ordering {
            std::cmp::Ordering::Greater
        }
    }
    impl Reifies for LT {
        type Output = std::cmp::Ordering;
        #[inline(always)]
        fn reflect(&self) -> std::cmp::Ordering {
            std::cmp::Ordering::Less
        }
    }
}
