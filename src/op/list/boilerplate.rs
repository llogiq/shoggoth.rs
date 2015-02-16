use list::*;
use syntax::product::{
    ToList,
    ToTuple,
};

impl Fn<(Nil,)> for ToList {
    type Output = Nil;
    #[inline]
    extern "rust-call" fn call(&self, (this,): (Nil,)) -> Nil {
        this
    }
}

impl<H, T: List> Fn<(Cons<H, T>,)> for ToList {
    type Output = Cons<H, T>;
    #[inline]
    extern "rust-call" fn call(&self, (this,): (Cons<H, T>,)) -> Cons<H, T> {
        this
    }
}

macro_rules! impl_to_tuple_for_seq {
    ($($seq:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($seq,)*> Fn<(List![$($seq),*],)> for ToTuple {
            type Output = ($($seq,)*);
            #[inline]
            extern "rust-call" fn call(&self, (this,): (List![$($seq),*],)) -> ($($seq,)*) {
                match this {
                    list_match![$($seq),*] => ($($seq,)*)
                }
            }
        }
    }
}

impl_for_seq_upto!{ impl_to_tuple_for_seq, 16 }
