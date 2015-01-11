use ty;

/// Predicate implemented when `Self` has a concept of `head` and `tail`
pub trait IsComposite {
    type H;
    type T;

    #[inline]
    fn split(self) -> (Self::H, Self::T);

    #[inline]
    fn head(self) -> Self::H where Self: Sized { self.split().0 }

    #[inline]
    fn tail(self) -> Self::T where Self: Sized { self.split().1 }
}

impl<A0,> IsComposite for (A0,) {
    type H = A0;
    type T = ();

    #[inline]
    fn split(self) -> (A0, (),) {
        (self.0, (),)
    }
}

impl<A0, A1,> IsComposite for (A0, A1,) {
    type H = A0;
    type T = (A1,);

    #[inline]
    fn split(self) -> (A0, (A1,),) {
        let (a0, a1,) = self;
        (a0, (a1,),)
    }
}

impl<A0, A1, A2,> IsComposite for (A0, A1, A2,) {
    type H = A0;
    type T = (A1, A2,);

    #[inline]
    fn split(self) -> (A0, (A1, A2,),) {
        let (a0, a1, a2,) = self;
        (a0, (a1, a2,),)
    }
}

impl<A0, A1, A2, A3,> IsComposite for (A0, A1, A2, A3,) {
    type H = A0;
    type T = (A1, A2, A3,);

     #[inline]
    fn split(self) -> (A0, (A1, A2, A3,),) {
        let (a0, a1, a2, a3,) = self;
        (a0, (a1, a2, a3,),)
    }
}

impl<A0, A1, A2, A3, A4,> IsComposite for (A0, A1, A2, A3, A4,) {
    type H = A0;
    type T = (A1, A2, A3, A4,);

    #[inline]
    fn split(self) -> (A0, (A1, A2, A3, A4,),) {
        let (a0, a1, a2, a3, a4,) = self;
        (a0, (a1, a2, a3, a4,),)
    }
}

impl<A0, A1, A2, A3, A4, A5,> IsComposite for (A0, A1, A2, A3, A4, A5,) {
    type H = A0;
    type T = (A1, A2, A3, A4, A5,);

    #[inline]
    fn split(self) -> (A0, (A1, A2, A3, A4, A5,),) {
        let (a0, a1, a2, a3, a4, a5,) = self;
        (a0, (a1, a2, a3, a4, a5,),)
    }
}

impl<A0, A1, A2, A3, A4, A5, A6,> IsComposite for (A0, A1, A2, A3, A4, A5, A6,) {
    type H = A0;
    type T = (A1, A2, A3, A4, A5, A6,);

    #[inline]
    fn split(self) -> (A0, (A1, A2, A3, A4, A5, A6,),) {
        let (a0, a1, a2, a3, a4, a5, a6,) = self;
        (a0, (a1, a2, a3, a4, a5, a6,),)
    }
}

impl<A0, A1, A2, A3, A4, A5, A6, A7,> IsComposite for (A0, A1, A2, A3, A4, A5, A6, A7,) {
    type H = A0;
    type T = (A1, A2, A3, A4, A5, A6, A7,);

    #[inline]
    fn split(self) -> (A0, (A1, A2, A3, A4, A5, A6, A7,),) {
        let (a0, a1, a2, a3, a4, a5, a6, a7,) = self;
        (a0, (a1, a2, a3, a4, a5, a6, a7,),)
    }
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8,> IsComposite for (A0, A1, A2, A3, A4, A5, A6, A7, A8,) {
    type H = A0;
    type T = (A1, A2, A3, A4, A5, A6, A7, A8,);

    #[inline]
    fn split(self) -> (A0, (A1, A2, A3, A4, A5, A6, A7, A8,),) {
        let (a0, a1, a2, a3, a4, a5, a6, a7, a8,) = self;
        (a0, (a1, a2, a3, a4, a5, a6, a7, a8,),)
    }
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9,> IsComposite for (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9,) {
    type H = A0;
    type T = (A1, A2, A3, A4, A5, A6, A7, A8, A9,);

     #[inline]
    fn split(self) -> (A0, (A1, A2, A3, A4, A5, A6, A7, A8, A9,),) {
        let (a0, a1, a2, a3, a4, a5, a6, a7, a8, a9,) = self;
        (a0, (a1, a2, a3, a4, a5, a6, a7, a8, a9,),)
    }
}

pub enum At<N: ty::Nat> {}

impl<A0> ty::Fn<(A0,)> for At<ty::_0> {
    type O = A0;
}
impl<A0> ty::DepFn<(A0,)> for At<ty::_0> {
    fn call<X>(arg: (A0,)) -> A0 {
        arg.0
    }
}

impl<A0, A1> ty::Fn<(A0, A1,)> for At<ty::_1> {
    type O = A1;
}
impl<A0, A1> ty::DepFn<(A0, A1,)> for At<ty::_1> {
    fn call<X>(arg: (A0, A1,)) -> A1 {
        arg.1
    }
}

impl<A0, A1, A2> ty::Fn<(A0, A1, A2,)> for At<ty::_2> {
    type O = A2;
}
impl<A0, A1, A2> ty::DepFn<(A0, A1, A2,)> for At<ty::_2> {
    fn call<X>(arg: (A0, A1, A2,)) -> A2 {
        arg.2
    }
}

impl<A0, A1, A2, A3> ty::Fn<(A0, A1, A2, A3,)> for At<ty::_3> {
    type O = A3;
}
impl<A0, A1, A2, A3> ty::DepFn<(A0, A1, A2, A3,)> for At<ty::_3> {
    fn call<X>(arg: (A0, A1, A2, A3,)) -> A3 {
        arg.3
    }
}

impl<A0, A1, A2, A3, A4> ty::Fn<(A0, A1, A2, A3, A4,)> for At<ty::_4> {
    type O = A4;
}
impl<A0, A1, A2, A3, A4> ty::DepFn<(A0, A1, A2, A3, A4,)> for At<ty::_4> {
    fn call<X>(arg: (A0, A1, A2, A3, A4,)) -> A4 {
        arg.4
    }
}

impl<A0, A1, A2, A3, A4, A5> ty::Fn<(A0, A1, A2, A3, A4, A5,)> for At<ty::_5> {
    type O = A5;
}
impl<A0, A1, A2, A3, A4, A5> ty::DepFn<(A0, A1, A2, A3, A4, A5,)> for At<ty::_5> {
    fn call<X>(arg: (A0, A1, A2, A3, A4, A5)) -> A5 {
        arg.5
    }
}

impl<A0, A1, A2, A3, A4, A5, A6> ty::Fn<(A0, A1, A2, A3, A4, A5, A6,)> for At<ty::_6> {
    type O = A6;
}
impl<A0, A1, A2, A3, A4, A5, A6> ty::DepFn<(A0, A1, A2, A3, A4, A5, A6,)> for At<ty::_6> {
    fn call<X>(arg: (A0, A1, A2, A3, A4, A5, A6,)) -> A6 {
        arg.6
    }
}

impl<A0, A1, A2, A3, A4, A5, A6, A7> ty::Fn<(A0, A1, A2, A3, A4, A5, A6, A7,)> for At<ty::_7> {
    type O = A7;
}
impl<A0, A1, A2, A3, A4, A5, A6, A7> ty::DepFn<(A0, A1, A2, A3, A4, A5, A6, A7,)> for At<ty::_7> {
    fn call<X>(arg: (A0, A1, A2, A3, A4, A5, A6, A7)) -> A7 {
        arg.7
    }
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8> ty::Fn<(A0, A1, A2, A3, A4, A5, A6, A7, A8,)> for At<ty::_8> {
    type O = A8;
}
impl<A0, A1, A2, A3, A4, A5, A6, A7, A8> ty::DepFn<(A0, A1, A2, A3, A4, A5, A6, A7, A8)> for At<ty::_8> {
    fn call<X>(arg: (A0, A1, A2, A3, A4, A5, A6, A7, A8)) -> A8 {
        arg.8
    }
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9> ty::Fn<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9,)> for At<ty::_9> {
    type O = A9;
}
impl<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9> ty::DepFn<(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9)> for At<ty::_9> {
    fn call<X>(arg: (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9)) -> A9 {
        arg.9
    }
}

/// Operations on `Tuples`
pub trait TupleOps {
    #[inline]
    fn head(self) -> <Self as IsComposite>::H where Self: Sized + IsComposite,
    {
        IsComposite::head(self)
    }

    #[inline]
    fn tail(self) -> <Self as IsComposite>::T where Self: Sized + IsComposite,
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
        At,
        TupleOps,
    };
    use ty::{
        self,
        DepMethod,
    };

    #[test]
    fn at() {
        println!("{:?}", (0u8,).dep::<At<ty::_0>>());
        println!("{:?}", (0u8, 1u8,).dep::<At<ty::_1>>());
        println!("{:?}", (0u8, 1u8, 2u8,).dep::<At<ty::_2>>());
        println!("{:?}", (0u8, 1u8, 2u8, 3u8,).dep::<At<ty::_3>>());
        println!("{:?}", (0u8, 1u8, 2u8, 3u8, 4u8,).dep::<At<ty::_4>>());
        println!("{:?}", (0u8, 1u8, 2u8, 3u8, 4u8, 5u8,).dep::<At<ty::_5>>());
        println!("{:?}", (0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8,).dep::<At<ty::_6>>());
        println!("{:?}", (0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8,).dep::<At<ty::_7>>());
        println!("{:?}", (0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8,).dep::<At<ty::_8>>());
        println!("{:?}", (0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8,).dep::<At<ty::_9>>());
    }

    #[test]
    fn head() {
        let x = (0u8,);
        x.head();

        let x = (0u8, 0u8,);
        x.head();

        let x = (0u8, 0u8, 0u8,);
        x.head();

        let x = (0u8, 0u8, 0u8, 0u8,);
        x.head();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8,);
        x.head();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.head();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.head();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.head();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.head();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.head();
    }

    #[test]
    fn tail() {
        let x = (0u8,);
        x.tail();

        let x = (0u8, 0u8,);
        x.tail();

        let x = (0u8, 0u8, 0u8,);
        x.tail();

        let x = (0u8, 0u8, 0u8, 0u8,);
        x.tail();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8,);
        x.tail();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.tail();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.tail();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.tail();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.tail();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.tail();
    }
}
