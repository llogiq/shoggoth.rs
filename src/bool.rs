#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FF;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TT;

pub trait Bool {
    fn as_bool(&self) -> bool;
}
impl Bool for FF {
    #[inline(always)]
    fn as_bool(&self) -> bool {
        false
    }
}
impl Bool for TT {
    #[inline(always)]
    fn as_bool(&self) -> bool {
        true
    }
}
