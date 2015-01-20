use hlist::*;

/// Trait for converting things to a `Tuple`
#[rustc_on_unimplemented = "`{Self}` cannot be converted to a tuple"]
pub trait ToTuple {
    type Out;
    #[inline]
    fn apply(self) -> Self::Out;
}

/// Trait for converting things to an `HList`
#[rustc_on_unimplemented = "`{Self}` cannot be converted to a heterogeneous list"]
pub trait ToHList {
    type Out: HList;
    #[inline]
    fn apply(self) -> Self::Out;
}

/// Operations on `Products`
#[rustc_on_unimplemented = "Product operations are not specified for `{Self}`"]
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
