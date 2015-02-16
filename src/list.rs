#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Nil;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Cons<H, T: List>(pub H, pub T);

#[rustc_on_unimplemented = "`{Self}` is not a heterogeneous list"]
pub trait List {}
impl List for Nil {}
impl<H, T: List> List for Cons<H, T> {}
