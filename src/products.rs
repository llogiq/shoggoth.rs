use hlist::{
    HCons,
    HList,
    HNil,
};

use tuples::{
    IsComposite,
};

pub trait ToTuple {
    type Out;
    #[inline]
    fn apply(self) -> Self::Out;
}

impl ToTuple for () {
    type Out = Self;
    #[inline]
    fn apply(self) -> Self { self }
}

impl<A0,> ToTuple for (A0,) {
    type Out = Self;
    #[inline]
    fn apply(self) -> Self { self }
}

impl<A0, A1,> ToTuple for (A0, A1,) {
    type Out = Self;
    #[inline]
    fn apply(self) -> Self { self }
}

impl<A0, A1, A2,> ToTuple for (A0, A1, A2,) {
    type Out = Self;
    #[inline]
    fn apply(self) -> Self { self }
}

impl<A0, A1, A2, A3,> ToTuple for (A0, A1, A2, A3,) {
    type Out = Self;
    #[inline]
    fn apply(self) -> Self { self }
}

impl<A0, A1, A2, A3, A4,> ToTuple for (A0, A1, A2, A3, A4,) {
    type Out = Self;
    #[inline]
    fn apply(self) -> Self { self }
}

impl<A0, A1, A2, A3, A4, A5,> ToTuple for (A0, A1, A2, A3, A4, A5,) {
    type Out = Self;
    #[inline]
    fn apply(self) -> Self { self }
}

impl<A0, A1, A2, A3, A4, A5, A6,> ToTuple for (A0, A1, A2, A3, A4, A5, A6,) {
    type Out = Self;
    #[inline]
    fn apply(self) -> Self { self }
}

impl<A0, A1, A2, A3, A4, A5, A6, A7,> ToTuple for (A0, A1, A2, A3, A4, A5, A6, A7,) {
    type Out = Self;
    #[inline]
    fn apply(self) -> Self { self }
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8,> ToTuple for (A0, A1, A2, A3, A4, A5, A6, A7, A8,) {
    type Out = Self;
    #[inline]
    fn apply(self) -> Self { self }
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9,> ToTuple for (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9,) {
    type Out = Self;
    #[inline]
    fn apply(self) -> Self { self }
}

pub trait ToHList {
    type Out: HList;
    #[inline]
    fn apply(self) -> Self::Out;
}

impl ToHList for () {
    type Out = HNil;
    #[inline]
    fn apply(self) -> HNil {
        HNil
    }
}

impl<A0: Copy,> ToHList for (A0,) {
    type Out = HCons<A0, HNil>;
    #[inline]
    fn apply(self) -> HCons<A0, HNil> {
        HCons(
            IsComposite::head(self),
            ToHList::apply(IsComposite::tail(self)),
        )
    }
}

impl<A0: Copy, A1: Copy,> ToHList for (A0, A1,) {
    type Out = HCons<A0, HCons<A1, HNil>>;
    #[inline]
    fn apply(self) -> HCons<A0, HCons<A1, HNil>> {
        HCons(
            IsComposite::head(self),
            ToHList::apply(IsComposite::tail(self)),
            )
    }
}

impl<A0: Copy, A1: Copy, A2: Copy,> ToHList for (A0, A1, A2,) {
    type Out = HCons<A0, HCons<A1, HCons<A2, HNil>>>;
    #[inline]
    fn apply(self) -> HCons<A0, HCons<A1, HCons<A2, HNil>>> {
        HCons(
            IsComposite::head(self),
            ToHList::apply(IsComposite::tail(self)),
            )
    }
}

impl<A0: Copy, A1: Copy, A2: Copy, A3: Copy,> ToHList for (A0, A1, A2, A3,) {
    type Out = HCons<A0, HCons<A1, HCons<A2, HCons<A3, HNil>>>>;
    #[inline]
    fn apply(self) -> HCons<A0, HCons<A1, HCons<A2, HCons<A3, HNil>>>> {
        HCons(
            IsComposite::head(self),
            ToHList::apply(IsComposite::tail(self)),
            )
    }
}

impl<A0: Copy, A1: Copy, A2: Copy, A3: Copy, A4: Copy,> ToHList for (A0, A1, A2, A3, A4,) {
    type Out = HCons<A0, HCons<A1, HCons<A2, HCons<A3, HCons<A4, HNil>>>>>;
    #[inline]
    fn apply(self) -> HCons<A0, HCons<A1, HCons<A2, HCons<A3, HCons<A4, HNil>>>>> {
        HCons(
            IsComposite::head(self),
            ToHList::apply(IsComposite::tail(self)),
            )
    }
}

impl<A0: Copy, A1: Copy, A2: Copy, A3: Copy, A4: Copy, A5: Copy,> ToHList for (A0, A1, A2, A3, A4, A5,) {
    type Out = HCons<A0, HCons<A1, HCons<A2, HCons<A3, HCons<A4, HCons<A5, HNil>>>>>>;
    #[inline]
    fn apply(self) -> HCons<A0, HCons<A1, HCons<A2, HCons<A3, HCons<A4, HCons<A5, HNil>>>>>> {
        HCons(
            IsComposite::head(self),
            ToHList::apply(IsComposite::tail(self)),
            )
    }
}

impl<A0: Copy, A1: Copy, A2: Copy, A3: Copy, A4: Copy, A5: Copy, A6: Copy,> ToHList for (A0, A1, A2, A3, A4, A5, A6,) {
    type Out = HCons<A0, HCons<A1, HCons<A2, HCons<A3, HCons<A4, HCons<A5, HCons<A6, HNil>>>>>>>;
    #[inline]
    fn apply(self) -> HCons<A0, HCons<A1, HCons<A2, HCons<A3, HCons<A4, HCons<A5, HCons<A6, HNil>>>>>>> {
        HCons(
            IsComposite::head(self),
            ToHList::apply(IsComposite::tail(self)),
            )
    }
}

impl<A0: Copy, A1: Copy, A2: Copy, A3: Copy, A4: Copy, A5: Copy, A6: Copy, A7: Copy,> ToHList for (A0, A1, A2, A3, A4, A5, A6, A7,) {
    type Out = HCons<A0, HCons<A1, HCons<A2, HCons<A3, HCons<A4, HCons<A5, HCons<A6, HCons<A7, HNil>>>>>>>>;
    #[inline]
    fn apply(self) -> HCons<A0, HCons<A1, HCons<A2, HCons<A3, HCons<A4, HCons<A5, HCons<A6, HCons<A7, HNil>>>>>>>> {
        HCons(
            IsComposite::head(self),
            ToHList::apply(IsComposite::tail(self)),
            )
    }
}

impl<A0: Copy, A1: Copy, A2: Copy, A3: Copy, A4: Copy, A5: Copy, A6: Copy, A7: Copy, A8: Copy,> ToHList for (A0, A1, A2, A3, A4, A5, A6, A7, A8,) {
    type Out = HCons<A0, HCons<A1, HCons<A2, HCons<A3, HCons<A4, HCons<A5, HCons<A6, HCons<A7, HCons<A8, HNil>>>>>>>>>;
    #[inline]
    fn apply(self) -> HCons<A0, HCons<A1, HCons<A2, HCons<A3, HCons<A4, HCons<A5, HCons<A6, HCons<A7, HCons<A8, HNil>>>>>>>>> {
        HCons(
            IsComposite::head(self),
            ToHList::apply(IsComposite::tail(self)),
            )
    }
}

impl<A0: Copy, A1: Copy, A2: Copy, A3: Copy, A4: Copy, A5: Copy, A6: Copy, A7: Copy, A8: Copy, A9: Copy,> ToHList for (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9,) {
    type Out = HCons<A0, HCons<A1, HCons<A2, HCons<A3, HCons<A4, HCons<A5, HCons<A6, HCons<A7, HCons<A8, HCons<A9, HNil>>>>>>>>>>;
    #[inline]
    fn apply(self) -> HCons<A0, HCons<A1, HCons<A2, HCons<A3, HCons<A4, HCons<A5, HCons<A6, HCons<A7, HCons<A8, HCons<A9, HNil>>>>>>>>>> {
        HCons(
            IsComposite::head(self),
            ToHList::apply(IsComposite::tail(self)),
            )
    }
}

pub trait ProductOps: Sized {
    #[inline]
    fn to_hlist(self) -> <Self as ToHList>::Out where
        Self: ToHList
    {
        ToHList::apply(self)
    }

    #[inline]
    fn to_tuple(self) -> <Self as ToTuple>::Out where
        Self: ToTuple
    {
        ToTuple::apply(self)
    }
}

impl ProductOps for () {}
impl<A0,> ProductOps for (A0,) {}
impl<A0, A1,> ProductOps for (A0, A1,) {}
impl<A0, A1, A2,> ProductOps for (A0, A1, A2,) {}
impl<A0, A1, A2, A3,> ProductOps for (A0, A1, A2, A3,) {}
impl<A0, A1, A2, A3, A4,> ProductOps for (A0, A1, A2, A3, A4,) {}
impl<A0, A1, A2, A3, A4, A5,> ProductOps for (A0, A1, A2, A3, A4, A5,) {}
impl<A0, A1, A2, A3, A4, A5, A6,> ProductOps for (A0, A1, A2, A3, A4, A5, A6,) {}
impl<A0, A1, A2, A3, A4, A5, A6, A7,> ProductOps for (A0, A1, A2, A3, A4, A5, A6, A7,) {}
impl<A0, A1, A2, A3, A4, A5, A6, A7, A8,> ProductOps for (A0, A1, A2, A3, A4, A5, A6, A7, A8,) {}
impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9,> ProductOps for (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9,) {}
