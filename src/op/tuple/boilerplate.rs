use syntax::product::{
    ToHList,
    ToTuple,
};
use op::tuple::{
    IsComposite,
};

macro_rules! impl_is_composite_for_seq {
    ($($seq:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($seq,)*> IsComposite for ($($seq,)*) {
            type H = seq_head!($($seq),*);
            type T = seq_tail!($($seq),*);
            #[inline]
            fn pair(self) -> (seq_head!($($seq),*), seq_tail!($($seq),*)) {
                let ($($seq,)*) = self;
                (seq_head!($($seq),*), seq_tail!($($seq),*))
            }
        }
    }
}
impl_for_seq_upto!{ impl_is_composite_for_seq, 16 }

macro_rules! impl_to_hlist_for_seq {
    ($($seq:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($seq,)*> Fn<(($($seq,)*),)> for ToHList {
            type Output = HList![$($seq),*];
            #[inline]
            extern "rust-call" fn call(&self, (arg,): (($($seq,)*),)) -> HList![$($seq),*] {
                match arg { ($($seq,)*) => hlist![$($seq),*] }
            }
        }
    }
}
impl_for_seq_upto!{ impl_to_hlist_for_seq, 16 }

macro_rules! impl_to_tuple_for_seq {
    ($($seq:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($seq,)*> Fn<(($($seq,)*),)> for ToTuple {
            type Output = ($($seq,)*);
            #[inline]
            extern "rust-call" fn call(&self, (arg,): (($($seq,)*),)) -> ($($seq,)*) {
                arg
            }
        }
    }
}
impl_for_seq_upto!{ impl_to_tuple_for_seq, 16 }
