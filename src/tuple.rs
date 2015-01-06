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
    fn head(self) -> A0 { self.0 }

    #[inline]
    fn tail(self) -> () { () }
}

impl<A0, A1,> IsComposite for (A0, A1,) {
    type H = A0;
    type T = (A1,);

    #[inline]
    fn head(self) -> A0 { self.0 }

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

pub trait TupleOps: Sized {
    #[inline]
    fn head(self) -> <Self as IsComposite>::H where Self: IsComposite
    {
        IsComposite::head(self)
    }

    #[inline]
    fn tail(self) -> <Self as IsComposite>::T where Self: IsComposite
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
        TupleOps,
    };

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
