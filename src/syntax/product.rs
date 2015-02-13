pub struct ToHList;
pub struct ToPair;
pub struct ToTuple;

pub trait ProductOps: Sized {
    #[inline]
    fn to_pair (self) -> <ToPair  as Fn<(Self,)>>::Output where
        ToPair: Fn<(Self,)>
    {
        ToPair .call((self,))
    }

    #[inline]
    fn to_hlist(self) -> <ToHList as Fn<(Self,)>>::Output where
        ToHList: Fn<(Self,)>
    {
        ToHList.call((self,))
    }

    #[inline]
    fn to_tuple(self) -> <ToTuple as Fn<(Self,)>>::Output where
        ToTuple: Fn<(Self,)>
    {
        ToTuple.call((self,))
    }
}
impl<Seq> ProductOps for Seq {}
