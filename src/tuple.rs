use hlist::*;
use product::{
    ToHList,
    ToTuple,
};
use ty;

/// Predicate implemented when `Self` has a concept of `head` and `tail`
#[rustc_on_unimplemented = "`{Self}` is not a composite type"]
pub trait IsComposite: Sized {
    type H;
    type T;

    #[inline]
    fn split(self) -> (Self::H, Self::T);

    #[inline]
    fn head(self) -> Self::H {
        self.split().0
    }

    #[inline]
    fn tail(self) -> Self::T {
        self.split().1
    }
}

trait At<N: ty::Tm<ty::nat::Nat>>: IsComposite {
    type Out;

    #[inline]
    fn at(self) -> Self::Out;
}

impl<
    Ctx0: IsComposite<H = H>,
    H,
> At<ty::_0b> for Ctx0 {
    type Out = H;

    #[inline]
    fn at(self) -> H {
        self.head()
    }
}

impl<
    Ctx0: IsComposite<T = Ctx1>,
    Ctx1: IsComposite<H = TH>,
    TH,
> At<ty::_1b> for Ctx0 {
    type Out = TH;

    #[inline]
    fn at(self) -> TH {
        self.tail().head()
    }
}

impl<
    Ctx0: IsComposite<T = Ctx1>,
    Ctx1: At<Rec0, Out = Rec1>,
    B: ty::Tm<ty::Bit>,
    P: ty::Tm<ty::nat::pos::Pos>,
    Rec0: ty::Tm<ty::nat::pos::Pos>,
    Rec1,
> At<(P, B)> for Ctx0 where
    (P, B): ty::Eval1<ty::nat::pos::Pred, Out = Rec0>,
{
    type Out = Rec1;

    #[inline]
    fn at(self) -> Rec1 {
        <Ctx1 as At<Rec0>>::at(self.tail())
    }
}

/// Operations on `Tuples`
#[rustc_on_unimplemented = "Tuple operations are not specified for `{Self}`"]
pub trait TupleOps: Sized {
    #[inline]
    fn at<N: ty::Tm<ty::nat::Nat>>(self) -> <Self as At<N>>::Out where Self: At<N> {
        <Self as At<N>>::at(self)
    }

    #[inline]
    fn head(self) -> <Self as IsComposite>::H where Self: IsComposite {
        IsComposite::head(self)
    }

    #[inline]
    fn tail(self) -> <Self as IsComposite>::T where Self: IsComposite {
        IsComposite::tail(self)
    }
}

impl<Ctx> TupleOps for Ctx {}

impl ToHList for () {
    type Out = HN;

    #[inline]
    fn apply(self) -> HN {
        Nil
    }
}
impl<
    H,
    T: ToHList,
    Ctx: IsComposite<H = H, T = T>,
> ToHList for Ctx {
    type Out = HC<H, <T as ToHList>::Out>;

    #[inline]
    fn apply(self) -> HC<H, <T as ToHList>::Out> {
        let (head, tail) = self.split();
        Cons(head, <T as ToHList>::apply(tail))
    }
}

impl<Ctx: TupleOps> ToTuple for Ctx {
    type Out = Self;

    #[inline]
    fn apply(self) -> Self {
        self
    }
}

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
impl_for_seq_upto!{ impl_is_composite_for_seq, 64 }

#[cfg(test)]
mod test {
    use hlist::*;
    use product::{
        ProductOps,
    };
    use tuple::{
        TupleOps,
    };
    use ty;

    type _3b = (ty::_1b, ty::_1b);

    #[test]
    fn at() {
        assert_eq!((0u8, true, "foo", Some(42u64), Ok::<_, ()>(false)).at::<_3b>(),
                   Some(42u64));
    }

    #[test]
    fn to_hlist() {
        assert_eq!((0u8, true, "foo", Some(42u64), Ok::<_, ()>(false)).to_hlist(),
                   Cons(0u8,
                   Cons(true,
                   Cons("foo",
                   Cons(Some(42u64),
                   Cons(Ok::<_, ()>(false),
                   Nil))))));
    }
}
