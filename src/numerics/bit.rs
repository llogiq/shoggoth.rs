use std::marker::{
    MarkerTrait,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct _0;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct _1;

pub trait Bit: MarkerTrait {}
impl Bit for _0 {}
impl Bit for _1 {}

#[cfg(test)]
mod test {
    #[test]
    fn macro_b_0() {
        let b: B![0] = b![0];
        match b {
            b![0] => {}
        }
    }

    #[test]
    fn macro_b() {
        let _: B![1,1,0,0,1,1] = b![1,1,0,0,1,1];
    }

    #[test]
    fn macro_b_dots() {
        let _: B![0,0,1,1,0,0] = b![..];
    }

    #[test]
    fn macro_b_pat() {
        let b: B![0,0,1,1,0,0] = b![..];
        match (b, b) {
            (b![..], b![0,0,1,1,0,0]) => {}
        }
    }
}
