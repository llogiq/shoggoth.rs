use hlist::*;

mod boilerplate;

pub trait IsHCons: HList {
    type H;
    type T;
    fn head(self) -> Self::H;
    fn tail(self) -> Self::T;
}

impl<H, T: HList> IsHCons for Cons<H, T> {
    type H = H;
    type T = T;

    #[inline]
    fn head(self) -> H {
        self.0
    }

    #[inline]
    fn tail(self) -> T {
        self.1
    }
}
