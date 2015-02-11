use ty;

mod boilerplate;

#[rustc_on_unimplemented = "`{Self}` is not a composite type"]
pub trait IsComposite: Sized {
    type H;
    type T;

    #[inline] fn split(self) -> (Self::H, Self::T);
    #[inline] fn head (self) -> Self::H { self.split().0 }
    #[inline] fn tail (self) -> Self::T { self.split().1 }
}

#[rustc_on_unimplemented = "Cannot get element of `{Self}` at index `{N}`"]
trait At<N>: IsComposite where N: ty::Tm<ty::nat::Nat> {
    type Out;

    #[inline] fn at(self) -> Self::Out;
}

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
impl<Seq> TupleOps for Seq {}

#[cfg(test)]
mod test {
    use hlist::*;
    use product::{
        ProductOps
    };
    use super::*;
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
