/// "Higher-kinded type" tags
pub mod tag {
    /// HKT tag for `Option`
    #[derive(Clone)]
    #[derive(Copy)]
    #[derive(Eq)]
    #[derive(Ord)]
    #[derive(PartialEq)]
    #[derive(PartialOrd)]
    #[derive(Show)]
    pub enum Option {}

    /// HKT tag for `Vec`
    #[derive(Clone)]
    #[derive(Copy)]
    #[derive(Eq)]
    #[derive(Ord)]
    #[derive(PartialEq)]
    #[derive(PartialOrd)]
    #[derive(Show)]
    pub enum Vec {}
}

/// Trait for projecting the parameter of a generic type
pub trait Arg { type O; }

/// Trait for producing the HKT tag for a generic type
pub trait Tag { type O; }

/// Trait for applying an HKT tag to an argument
pub trait Ins<T> { type O: Arg<O = Self> + Tag<O = T>; }

/// Type-level function for applying an HKT tag to a parameter
pub type Ap<T, A: Ins<T>> = <A as Ins<T>>::O;
pub type Un<TX: Arg> = <TX as Arg>::O;

// Option as an HKT
impl<A> Arg for Option<A> { type O = A; }
impl<A> Tag for Option<A> { type O = tag::Option; }
impl<A> Ins<tag::Option> for A { type O = Option<A>; }

// Vec as an HKT
impl<A> Arg for Vec<A> { type O = A; }
impl<A> Tag for Vec<A> { type O = tag::Vec; }
impl<A> Ins<tag::Vec> for A { type O = Vec<A>; }
