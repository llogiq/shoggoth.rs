use op::list::{
    IsCons,
};

pub trait ListOps {
    #[inline]
    fn head(self) -> <Self as IsCons>::H where
        Self: Sized + IsCons,
    {
        IsCons::head(self)
    }

    #[inline]
    fn tail(self) -> <Self as IsCons>::T where
        Self: Sized + IsCons,
    {
        IsCons::tail(self)
    }
}

impl<Seq> ListOps for Seq {
}
