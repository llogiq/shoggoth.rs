use std::marker::{
    PhantomData
};

pub trait Reifies {
    type Output;
    fn reflect(&self) -> Self::Output;
}

impl<A, Rec> Reifies for PhantomData<A> where
    A: Reifies<Output = Rec>
{
    type Output = Rec;
    fn reflect(&self) -> Rec {
        unsafe { ::std::mem::transmute::<&PhantomData<A>, &A>(self).reflect() }
    }
}
