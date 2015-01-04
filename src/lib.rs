#![feature(associated_types)]

///
// pub trait DepFn0 {
//     type Out;
//     fn apply() -> Self;
// }

// pub trait DepFn1 {
//     type Out;
//     fn apply(self) -> Self::Out;
// }

// pub trait DepFn2<U> {
//     type Out;
//     fn apply(self, u: U) -> Self::Out;
// }
///

pub trait HList {}

#[deriving(Copy)]
#[deriving(Show)]
pub struct HNil;
impl HList for HNil {}

#[deriving(Copy)]
#[deriving(Show)]
pub struct HCons<H, T: HList>(H, T);
impl<H, T: HList> HList for HCons<H, T> {}
///
pub trait IsHCons: HList {
    type H;
    type T: HList;

    #[inline]
    fn head(self) -> Self::H;
    #[inline]
    fn tail(self) -> Self::T;
}

impl<H, T: HList> IsHCons for HCons<H, T> {
    type H = H;
    type T = T;

    #[inline]
    fn head(self) -> H { self.0 }
    #[inline]
    fn tail(self) -> T { self.1 }
}
///
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
///
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

///
pub trait ProductOps {
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

///
pub trait IsComposite {
    type H;
    type T;

    #[inline]
    fn head(self) -> Self::H;
    #[inline]
    fn tail(self) -> Self::T;
}

impl<A0,> IsComposite for (A0,) {
    type H = A0;
    type T = ();

    #[inline]
    fn head(self) -> A0 {
        self.0
    }

    #[inline]
    fn tail(self) -> () {
        ()
    }
}

impl<A0, A1,> IsComposite for (A0, A1,) {
    type H = A0;
    type T = (A1,);

    #[inline]
    fn head(self) -> A0 {
        self.0
    }

    #[inline]
    fn tail(self) -> (A1,) {
        let (_, a1,) = self;
        (a1,)
    }
}

impl<A0, A1, A2,> IsComposite for (A0, A1, A2,) {
    type H = A0;
    type T = (A1, A2,);

    #[inline]
    fn head(self) -> A0 { self.0 }

    #[inline]
    fn tail(self) -> (A1, A2,) {
        let (_, a1, a2,) = self;
        (a1, a2,)
    }
}

impl<A0, A1, A2, A3,> IsComposite for (A0, A1, A2, A3,) {
    type H = A0;
    type T = (A1, A2, A3,);

    #[inline]
    fn head(self) -> A0 { self.0 }

    #[inline]
    fn tail(self) -> (A1, A2, A3,) {
        let (_, a1, a2, a3,) = self;
        (a1, a2, a3,)
    }
}

impl<A0, A1, A2, A3, A4> IsComposite for (A0, A1, A2, A3, A4,) {
    type H = A0;
    type T = (A1, A2, A3, A4,);

    #[inline]
    fn head(self) -> A0 { self.0 }

    #[inline]
    fn tail(self) -> (A1, A2, A3, A4,) {
        let (_, a1, a2, a3, a4,) = self;
        (a1, a2, a3, a4,)
    }
}

impl<A0, A1, A2, A3, A4, A5,> IsComposite for (A0, A1, A2, A3, A4, A5,) {
    type H = A0;
    type T = (A1, A2, A3, A4, A5,);

    #[inline]
    fn head(self) -> A0 { self.0 }

    #[inline]
    fn tail(self) -> (A1, A2, A3, A4, A5,) {
        let (_, a1, a2, a3, a4, a5,) = self;
        (a1, a2, a3, a4, a5,)
    }
}

impl<A0, A1, A2, A3, A4, A5, A6,> IsComposite for (A0, A1, A2, A3, A4, A5, A6,) {
    type H = A0;
    type T = (A1, A2, A3, A4, A5, A6,);

    #[inline]
    fn head(self) -> A0 { self.0 }

    #[inline]
    fn tail(self) -> (A1, A2, A3, A4, A5, A6,) {
        let (_, a1, a2, a3, a4, a5, a6,) = self;
        (a1, a2, a3, a4, a5, a6,)
    }
}

impl<A0, A1, A2, A3, A4, A5, A6, A7,> IsComposite for (A0, A1, A2, A3, A4, A5, A6, A7,) {
    type H = A0;
    type T = (A1, A2, A3, A4, A5, A6, A7,);

    #[inline]
    fn head(self) -> A0 { self.0 }

    #[inline]
    fn tail(self) -> (A1, A2, A3, A4, A5, A6, A7,) {
        let (_, a1, a2, a3, a4, a5, a6, a7,) = self;
        (a1, a2, a3, a4, a5, a6, a7,)
    }
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8,> IsComposite for (A0, A1, A2, A3, A4, A5, A6, A7, A8,) {
    type H = A0;
    type T = (A1, A2, A3, A4, A5, A6, A7, A8,);

    #[inline]
    fn head(self) -> A0 { self.0 }

    #[inline]
    fn tail(self) -> (A1, A2, A3, A4, A5, A6, A7, A8,) {
        let (_, a1, a2, a3, a4, a5, a6, a7, a8,) = self;
        (a1, a2, a3, a4, a5, a6, a7, a8,)
    }
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9,> IsComposite for (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9,) {
    type H = A0;
    type T = (A1, A2, A3, A4, A5, A6, A7, A8, A9,);

    #[inline]
    fn head(self) -> A0 { self.0 }

    #[inline]
    fn tail(self) -> (A1, A2, A3, A4, A5, A6, A7, A8, A9,) {
        let (_, a1, a2, a3, a4, a5, a6, a7, a8, a9,) = self;
        (a1, a2, a3, a4, a5, a6, a7, a8, a9,)
    }
}

///
pub trait TupleOps {
    #[inline]
    fn head(self) -> <Self as IsComposite>::H where
        Self: IsComposite
    {
        IsComposite::head(self)
    }

    #[inline]
    fn tail(self) -> <Self as IsComposite>::T where
        Self: IsComposite
    {
        IsComposite::tail(self)
    }
}

impl TupleOps for () {}
impl<A0,> TupleOps for (A0,) {}
impl<A0, A1,> TupleOps for (A0, A1,) {}
impl<A0, A1, A2,> TupleOps for (A0, A1, A2,) {}
impl<A0, A1, A2, A3,> TupleOps for (A0, A1, A2, A3,) {}
impl<A0, A1, A2, A3, A4,> TupleOps for (A0, A1, A2, A3, A4,) {}
impl<A0, A1, A2, A3, A4, A5,> TupleOps for (A0, A1, A2, A3, A4, A5,) {}
impl<A0, A1, A2, A3, A4, A5, A6,> TupleOps for (A0, A1, A2, A3, A4, A5, A6,) {}
impl<A0, A1, A2, A3, A4, A5, A6, A7,> TupleOps for (A0, A1, A2, A3, A4, A5, A6, A7,) {}
impl<A0, A1, A2, A3, A4, A5, A6, A7, A8,> TupleOps for (A0, A1, A2, A3, A4, A5, A6, A7, A8,) {}
impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9,> TupleOps for (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9,) {}

#[cfg(test)]
mod tests {
    use super::{
        ProductOps,
        TupleOps,
    };

    #[test]
    fn test01() {
        let x = (0u8,);
        println!("{}", x.head());
        println!("{}", x.tail());
        println!("{}", x.to_hlist());

        let x = (0u8, 0u8,);
        println!("{}", x.head());
        println!("{}", x.tail());
        println!("{}", x.to_hlist());

        let x = (0u8, 0u8, 0u8,);
        println!("{}", x.head());
        println!("{}", x.tail());
        println!("{}", x.to_hlist());

        let x = (0u8, 0u8, 0u8, 0u8,);
        println!("{}", x.head());
        println!("{}", x.tail());
        println!("{}", x.to_hlist());

        let x = (0u8, 0u8, 0u8, 0u8, 0u8,);
        println!("{}", x.head());
        println!("{}", x.tail());
        println!("{}", x.to_hlist());

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        println!("{}", x.head());
        println!("{}", x.tail());
        println!("{}", x.to_hlist());

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        println!("{}", x.head());
        println!("{}", x.tail());
        println!("{}", x.to_hlist());

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        println!("{}", x.head());
        println!("{}", x.tail());
        println!("{}", x.to_hlist());

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        println!("{}", x.head());
        println!("{}", x.tail());
        println!("{}", x.to_hlist());

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        println!("{}", x.head());
        println!("{}", x.tail());
        println!("{}", x.to_hlist());
    }
}

