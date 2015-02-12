use product::{
    ToHList,
    ToTuple,
};
use tuple::{
    At,
    IsComposite,
};
use ty;

macro_rules! impl_is_composite_for_seq {
    ($($seq:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($seq,)*> IsComposite for ($($seq,)*) {
            type H = seq_head!($($seq),*);
            type T = seq_tail!($($seq),*);
            #[inline]
            fn split(self) -> (seq_head!($($seq),*), seq_tail!($($seq),*)) {
                let ($($seq,)*) = self;
                (seq_head!($($seq),*), seq_tail!($($seq),*))
            }
        }
    }
}
impl_for_seq_upto!{ impl_is_composite_for_seq, 32 }

impl<H, Seq0> At<ty::_0b> for Seq0 where Seq0: IsComposite<H = H> {
    type Out = H;
    #[inline] fn at(self) -> H { self.head() }
}
impl<Seq0, Seq1, TH> At<ty::_1b> for Seq0 where
    Seq0: IsComposite<T = Seq1>,
    Seq1: IsComposite<H = TH>,
{
    type Out = TH;
    #[inline] fn at(self) -> TH { self.tail().head() }
}
impl<
       B,
       P: ty::nat::pos::Pos,
    Rec0: ty::nat::pos::Pos,
    Rec1,
    Seq0,
    Seq1,
> At<(P, B)> for Seq0 where
    Seq0: IsComposite<T = Seq1>,
    Seq1: At<Rec0, Out = Rec1>,
    (P, B): ty::Eval1<ty::nat::pos::Pred, Out = Rec0>,
{
    type Out = Rec1;
    #[inline] fn at(self) -> Rec1 { <Seq1 as At<Rec0>>::at(self.tail()) }
}

macro_rules! impl_to_hlist_for_seq {
    ($($seq:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($seq,)*> ToHList for ($($seq,)*) {
            type Out = HList![$($seq),*];
            #[inline]
            fn apply(self) -> HList![$($seq),*] {
                match self { ($($seq,)*) => hlist![$($seq),*] }
            }
        }
    }
}
impl_for_seq_upto!{ impl_to_hlist_for_seq, 32 }

macro_rules! impl_to_tuple_for_seq {
    ($($seq:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($seq,)*> ToTuple for ($($seq,)*) {
            type Out = Self;
            #[inline]
            fn apply(self) -> Self { self }
        }
    }
}
impl_for_seq_upto!{ impl_to_tuple_for_seq, 32 }
