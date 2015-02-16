use op::list::{
    IsCons,
};

pub trait ListOps: Sized {
    #[inline]
    fn head(self) -> <Self as IsCons>::H where Self: IsCons {
        IsCons::head(self)
    }

    #[inline]
    fn tail(self) -> <Self as IsCons>::T where Self: IsCons {
        IsCons::tail(self)
    }
}
impl<Seq> ListOps for Seq {}
