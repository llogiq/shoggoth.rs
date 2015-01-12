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

#[cfg(test)]
mod tests {
}
