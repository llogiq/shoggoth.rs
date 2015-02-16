pub struct ToList;
pub struct ToTuple;

pub trait ProductOps: Sized {
    #[inline]
    fn to_list(self) -> <ToList as Fn<(Self,)>>::Output where ToList: Fn<(Self,)> {
        ToList.call((self,))
    }

    #[inline]
    fn to_tuple(self) -> <ToTuple as Fn<(Self,)>>::Output where ToTuple: Fn<(Self,)> {
        ToTuple.call((self,))
    }
}
impl<Seq> ProductOps for Seq {}
