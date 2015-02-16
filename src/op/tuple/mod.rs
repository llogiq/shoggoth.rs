mod boilerplate;

pub trait IsPair: Sized {
    type H;
    type T;

    fn pair(self) -> (Self::H, Self::T);

    #[inline]
    fn head(self) -> Self::H {
        self.pair().0
    }

    #[inline]
    fn tail(self) -> Self::T {
        self.pair().1
    }
}
