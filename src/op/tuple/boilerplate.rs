use nat;
use syntax::product::{
    ToHList,
    ToPair,
    ToTuple,
};
use op::tuple::{
    At,
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

impl<H, Seq0> Fn<(Seq0,)> for At<nat::_0> where
    Seq0: IsComposite<H = H>
{
    type Output = H;
    #[inline]
    extern "rust-call" fn call(&self, (this,): (Seq0,)) -> H {
        this.head()
    }
}
impl<Seq0, Seq1, TH> Fn<(Seq0,)> for At<nat::_1> where
    Seq0: IsComposite<T = Seq1>,
    Seq1: IsComposite<H = TH>,
{
    type Output = TH;
    #[inline]
    extern "rust-call" fn call(&self, (this,): (Seq0,)) -> TH {
        this.tail().head()
    }
}

// impl<
//        B,
//        P: ty::nat::pos::Pos,
//     Rec0: ty::nat::pos::Pos,
//     Rec1,
//     Seq0,
//     Seq1,
// > Fn<(Seq0,)> for At<(P, B)> where
//     Seq0: IsComposite<T = Seq1>,
//     Fn<(Seq1,), Output = Rec1>: At<Rec0>,
//     (P, B): ty::Eval1<ty::nat::pos::Pred, Out = Rec0>,
// {
//     type Output = Rec1;
//     #[inline]
//     extern "rust-call" fn call(&self, (this,): (Seq0,)) -> Rec1 {
//         At::<Rec0>(this.tail())
//     }
// }

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

impl<Seq: IsComposite> Fn<(Seq,)> for ToPair {
    type Output = (<Seq as IsComposite>::H, <Seq as IsComposite>::T);
    #[inline]
    extern "rust-call" fn call(&self, (arg,): (Seq,)) -> (<Seq as IsComposite>::H, <Seq as IsComposite>::T) {
        arg.pair()
    }
}

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
