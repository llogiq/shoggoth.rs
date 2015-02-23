use reflect::{
    Reifies,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FF;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TT;

impl Reifies for FF {
    type Output = bool;
    #[inline(always)]
    fn reflect(&self) -> bool {
        false
    }
}
impl Reifies for TT {
    type Output = bool;
    #[inline(always)]
    fn reflect(&self) -> bool {
        true
    }
}
