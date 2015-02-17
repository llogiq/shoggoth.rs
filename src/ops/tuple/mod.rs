mod boilerplate;

pub trait IsPair {
    type H;
    type T;

    fn pair(self) -> (Self::H, Self::T);

    #[inline]
    fn head(self) -> Self::H where
        Self: Sized,
    {
        self.pair().0
    }

    #[inline]
    fn tail(self) -> Self::T where
        Self: Sized,
    {
        self.pair().1
    }
}
