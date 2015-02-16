use op::tuple::{
    IsPair,
};

pub trait TupleOps: Sized {
    #[inline]
    fn head(self) -> <Self as IsPair>::H where Self: IsPair {
        IsPair::head(self)
    }

    #[inline]
    fn tail(self) -> <Self as IsPair>::T where Self: IsPair {
        IsPair::tail(self)
    }
}
impl<Seq> TupleOps for Seq {}
