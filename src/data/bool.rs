#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FF;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TT;

mod reflection {
    use data::reflect::*;
    use super::*;

    impl Reifies<Recursive> for FF {
        type Output = bool;
        fn reflect(&self) -> bool {
            false
        }
    }

    impl Reifies<Recursive> for TT {
        type Output = bool;
        fn reflect(&self) -> bool {
            true
        }
    }
}
