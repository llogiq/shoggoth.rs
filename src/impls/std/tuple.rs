use ops::tuple::{
    IsPair,
};

pub trait TupleOps {
    #[inline]
    fn head(self) -> <Self as IsPair>::H where
        Self: Sized + IsPair,
    {
        IsPair::head(self)
    }

    #[inline]
    fn tail(self) -> <Self as IsPair>::T where
        Self: Sized + IsPair,
    {
        IsPair::tail(self)
    }
}

impl<Seq> TupleOps for Seq {
}
