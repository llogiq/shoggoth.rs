pub mod bool;
pub mod fin;
pub mod fun;
pub mod list;
pub mod literal;
pub mod nat;
pub mod val;

/// Helper for calling "dependent functions" as methods on suitable types
pub trait DepMethod {
    #[inline(always)]
    fn dep<DF>(self) -> fun::Ap<DF, Self> where Self: Sized, DF: fun::DepFn<Self>,
    {
        fun::DepFn::call::<DF>(self)
    }
}

impl<A> DepMethod for A {}
