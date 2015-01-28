/// Predicate implemented when `Self` has a concept of `head` and `tail`
#[rustc_on_unimplemented = "`{Self}` is not a composite type"]
pub trait
    IsComposite
{
    type H;
    type T;

    #[inline]
    fn split(self) -> (Self::H, Self::T);

    #[inline]
    fn head(self) -> Self::H where
        Self: Sized
    {
        self.split().0
    }

    #[inline]
    fn tail(self) -> Self::T where
        Self: Sized
    {
        self.split().1
    }
}

impl<
    A0,
>
    IsComposite
for
    (A0,)
{
    type H = A0;
    type T = ();

    #[inline]
    fn split(self) -> (A0, ()) {
        let (a0,) = self;
        (a0, ())
    }
}

impl<
    A0,
    A1,
>
    IsComposite
for
    (A0, A1)
{
    type H = A0;
    type T = (A1,);

    #[inline]
    fn split(self) -> (A0, (A1,)) {
        let (a0, a1) = self;
        (a0, (a1,))
    }
}

impl<
    A0,
    A1,
    A2,
>
    IsComposite
for
    (A0, A1, A2)
{
    type H = A0;
    type T = (A1, A2);

    #[inline]
    fn split(self) -> (A0, (A1, A2)) {
        let (a0, a1, a2) = self;
        (a0, (a1, a2))
    }
}

impl<
    A0,
    A1,
    A2,
    A3,
>
    IsComposite
for
    (A0, A1, A2, A3)
{
    type H = A0;
    type T = (A1, A2, A3);

    #[inline]
    fn split(self) -> (A0, (A1, A2, A3)) {
        let (a0, a1, a2, a3) = self;
        (a0, (a1, a2, a3))
    }
}

/// Operations on `Tuples`
#[rustc_on_unimplemented = "Tuple operations are not specified for `{Self}`"]
pub trait
    TupleOps
{
    #[inline]
    fn head(self) -> <Self as IsComposite>::H where
        Self: Sized,
        Self: IsComposite,
    {
        IsComposite::head(self)
    }

    #[inline]
    fn tail(self) -> <Self as IsComposite>::T where
        Self: Sized,
        Self: IsComposite,
    {
        IsComposite::tail(self)
    }
}



#[cfg(test)]
mod test {
}
