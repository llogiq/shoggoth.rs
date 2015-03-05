#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FF;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TT;

mod reflection {
    use data::reflect::*;
    use super::*;

    impl Reflects for bool {
        fn reify(&self) -> Box<Reifies<Output = Self>> {
            match self {
                &false => { box FF }
                &true  => { box TT }
            }
        }
    }

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
