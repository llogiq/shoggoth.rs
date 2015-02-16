use op::tuple::{
    At,
    IsComposite,
};

pub trait HListOps: Sized {
    #[inline]
    fn head(self) -> <Self as IsComposite>::H where
        Self: IsComposite
    {
        IsComposite::head(self)
    }

    #[inline]
    fn tail(self) -> <Self as IsComposite>::T where
        Self: IsComposite
    {
        IsComposite::tail(self)
    }
}
impl<Seq> HListOps for Seq {}
