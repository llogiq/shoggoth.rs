mod boilerplate;

pub trait IsComposite: Sized {
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
