use list::*;

mod boilerplate;

pub trait IsCons: List {
    type H;
    type T;
    fn head(self) -> Self::H;
    fn tail(self) -> Self::T;
}

impl<H, T: List> IsCons for Cons<H, T> {
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
