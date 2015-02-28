#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FF;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TT;

mod reflection {
    use reflect::Reifies;
    use super::*;

    impl Reifies for FF {
        type Output = bool;
        fn reflect(&self) -> bool {
            false
        }
    }
    impl Reifies for TT {
        type Output = bool;
        fn reflect(&self) -> bool {
            true
        }
    }
}
