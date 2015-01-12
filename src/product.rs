use hlist::*;

/// Trait for converting things to a `Tuple`
pub trait ToTuple {
    type Out;
    #[inline]
    fn apply(self) -> Self::Out;
}

/// Trait for converting things to an `HList`
pub trait ToHList {
    type Out: HList;
    #[inline]
    fn apply(self) -> Self::Out;
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

#[cfg(test)]
mod tests {
}
