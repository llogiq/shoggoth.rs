use ty::{
    Ty,
};

/// Predicate classifying type-level type-checkable terms
#[rustc_on_unimplemented = "`{Self}` is not a valid type-level type-checkable term of type `{A}`"]
pub trait
    Tm<A>
where
    A: Ty,
{}
