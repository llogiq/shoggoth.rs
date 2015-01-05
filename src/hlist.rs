pub trait HList {}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Rand)]
#[derive(Show)]
pub struct HNil;
impl HList for HNil {}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Rand)]
#[derive(Show)]
pub struct HCons<H, T: HList>(pub H, pub T);
impl<H, T: HList> HList for HCons<H, T> {}

pub trait IsHCons: HList {
    type H;
    type T: HList;

    #[inline]
    fn head(self) -> Self::H;

    #[inline]
    fn tail(self) -> Self::T;
}

impl<H, T: HList> IsHCons for HCons<H, T> {
    type H = H;
    type T = T;

    #[inline]
    fn head(self) -> H { self.0 }

    #[inline]
    fn tail(self) -> T { self.1 }
}
