#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Nil;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Cons<H, T>(pub H, pub T);

#[rustc_on_unimplemented = "`{Self}` is not a heterogeneous list"]
pub trait HList {}
impl HList for Nil {}
impl<H, T: HList> HList for Cons<H, T> {}
