use hlist::{
    HC,
    HList,
    HN,
};

use tuple::{
    IsComposite,
};

/// Trait for converting things to a `Tuple`
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

/// Trait for converting things to an `HList`
pub trait ToHList {
    type Out: HList;
    #[inline]
    fn apply(self) -> Self::Out;
}

impl ToHList for () {
    type Out = HN;
    #[inline]
    fn apply(self) -> HN { HN }
}

impl<A0,> ToHList for (A0,) {
    type Out = HC<A0, HN>;
    #[inline]
    fn apply(self) -> HC<A0, HN> {
        let (h, t) = IsComposite::split(self);
        HC(h, ToHList::apply(t))
    }
}

impl<A0, A1,> ToHList for (A0, A1,) {
    type Out = HC<A0, HC<A1, HN>>;
    #[inline]
    fn apply(self) -> HC<A0, HC<A1, HN>> {
        let (h, t) = IsComposite::split(self);
        HC(h, ToHList::apply(t))
    }
}

impl<A0, A1, A2,> ToHList for (A0, A1, A2,) {
    type Out = HC<A0, HC<A1, HC<A2, HN>>>;
    #[inline]
    fn apply(self) -> HC<A0, HC<A1, HC<A2, HN>>> {
        let (h, t) = IsComposite::split(self);
        HC(h, ToHList::apply(t))
    }
}

impl<A0, A1, A2, A3,> ToHList for (A0, A1, A2, A3,) {
    type Out = HC<A0, HC<A1, HC<A2, HC<A3, HN>>>>;
    #[inline]
    fn apply(self) -> HC<A0, HC<A1, HC<A2, HC<A3, HN>>>> {
        let (h, t) = IsComposite::split(self);
        HC(h, ToHList::apply(t))
    }
}

impl<A0, A1, A2, A3, A4,> ToHList for (A0, A1, A2, A3, A4,) {
    type Out = HC<A0, HC<A1, HC<A2, HC<A3, HC<A4, HN>>>>>;
    #[inline]
    fn apply(self) -> HC<A0, HC<A1, HC<A2, HC<A3, HC<A4, HN>>>>> {
        let (h, t) = IsComposite::split(self);
        HC(h, ToHList::apply(t))
    }
}

impl<A0, A1, A2, A3, A4, A5,> ToHList for (A0, A1, A2, A3, A4, A5,) {
    type Out = HC<A0, HC<A1, HC<A2, HC<A3, HC<A4, HC<A5, HN>>>>>>;
    #[inline]
    fn apply(self) -> HC<A0, HC<A1, HC<A2, HC<A3, HC<A4, HC<A5, HN>>>>>> {
        let (h, t) = IsComposite::split(self);
        HC(h, ToHList::apply(t))
    }
}

impl<A0, A1, A2, A3, A4, A5, A6,> ToHList for (A0, A1, A2, A3, A4, A5, A6,) {
    type Out = HC<A0, HC<A1, HC<A2, HC<A3, HC<A4, HC<A5, HC<A6, HN>>>>>>>;
    #[inline]
    fn apply(self) -> HC<A0, HC<A1, HC<A2, HC<A3, HC<A4, HC<A5, HC<A6, HN>>>>>>> {
        let (h, t) = IsComposite::split(self);
        HC(h, ToHList::apply(t))
    }
}

impl<A0, A1, A2, A3, A4, A5, A6, A7,> ToHList for (A0, A1, A2, A3, A4, A5, A6, A7,) {
    type Out = HC<A0, HC<A1, HC<A2, HC<A3, HC<A4, HC<A5, HC<A6, HC<A7, HN>>>>>>>>;
    #[inline]
    fn apply(self) -> HC<A0, HC<A1, HC<A2, HC<A3, HC<A4, HC<A5, HC<A6, HC<A7, HN>>>>>>>> {
        let (h, t) = IsComposite::split(self);
        HC(h, ToHList::apply(t))
    }
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8,> ToHList for (A0, A1, A2, A3, A4, A5, A6, A7, A8,) {
    type Out = HC<A0, HC<A1, HC<A2, HC<A3, HC<A4, HC<A5, HC<A6, HC<A7, HC<A8, HN>>>>>>>>>;
    #[inline]
    fn apply(self) -> HC<A0, HC<A1, HC<A2, HC<A3, HC<A4, HC<A5, HC<A6, HC<A7, HC<A8, HN>>>>>>>>> {
        let (h, t) = IsComposite::split(self);
        HC(h, ToHList::apply(t))
    }
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9,> ToHList for (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9,) {
    type Out = HC<A0, HC<A1, HC<A2, HC<A3, HC<A4, HC<A5, HC<A6, HC<A7, HC<A8, HC<A9, HN>>>>>>>>>>;
    #[inline]
    fn apply(self) -> HC<A0, HC<A1, HC<A2, HC<A3, HC<A4, HC<A5, HC<A6, HC<A7, HC<A8, HC<A9, HN>>>>>>>>>> {
        let (h, t) = IsComposite::split(self);
        HC(h, ToHList::apply(t))
    }
}

/// Operations on `Products`
pub trait ProductOps {
    #[inline]
    fn to_hlist(self) -> <Self as ToHList>::Out where
        Self: Sized + ToHList,
    {
        ToHList::apply(self)
    }

    #[inline]
    fn to_tuple(self) -> <Self as ToTuple>::Out where
        Self: Sized + ToTuple,
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

#[cfg(test)]
mod tests {
    use super::{
        ProductOps,
    };

    #[test]
    fn to_hlist() {
        let x = (0u8,);
        x.to_hlist();

        let x = (0u8, 0u8,);
        x.to_hlist();

        let x = (0u8, 0u8, 0u8,);
        x.to_hlist();

        let x = (0u8, 0u8, 0u8, 0u8,);
        x.to_hlist();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8,);
        x.to_hlist();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.to_hlist();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.to_hlist();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.to_hlist();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.to_hlist();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.to_hlist();
    }
}
