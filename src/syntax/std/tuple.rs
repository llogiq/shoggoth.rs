use op::tuple::{
    At,
    IsComposite,
};

pub trait TupleOps: Sized {
    #[inline]
    fn at<N>(self) -> <At<N> as Fn<(Self,)>>::Output where At<N>: Fn<(Self,)> {
        At::<N>(self)
    }

    #[inline]
    fn drop(self) {
    }

    #[inline]
    fn head(self) -> <Self as IsComposite>::H where Self: IsComposite {
        IsComposite::head(self)
    }

    #[inline]
    fn init(self) {
    }

    #[inline]
    fn last(self) {
    }

    #[inline]
    fn length(self) {
    }

    #[inline]
    fn map(self) {
    }

    #[inline]
    fn flat_map(self) {
    }

    #[inline]
    fn prepend(self) {
    }

    #[inline]
    fn replace_at(self) {
    }

    #[inline]
    fn reverse(self) {
    }

    #[inline]
    fn split(self) {
    }

    #[inline]
    fn tail(self) -> <Self as IsComposite>::T where Self: IsComposite {
        IsComposite::tail(self)
    }

    #[inline]
    fn take(self) {
    }
}

impl<Seq> TupleOps for Seq {}
