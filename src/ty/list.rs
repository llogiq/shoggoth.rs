use ty::fun;
use ty::nat;

/// Type-level empty list
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Nil {}

/// Type-level cons list
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Cons<H, T: List> {}

/// Predicate classifying type-level lists
pub trait List {}
impl List for Nil {}
impl<H, T: List> List for Cons<H, T> {}

/// Type-level function for list append
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Append {}
impl<RHS: List> fun::Fn<(Nil, RHS,)> for Append
{
    type O = RHS;
}
impl<H, LHS: List, RHS: List> fun::Fn<(Cons<H, LHS>, RHS,)> for Append where
    Append: fun::Fn<(LHS, RHS,)>,
{
    type O = Cons<H, fun::Ap<Append, (LHS, RHS,)>>;
}

/// Type-level function for list length
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Length {}
impl fun::Fn<(Nil,)> for Length {
    type O = nat::Z;
}
impl<H, T: List> fun::Fn<(Cons<H, T>,)> for Length where
    Length: fun::Fn<(T,)>,
{
    type O = nat::S<fun::Ap<Length, (T,)>>;
}

#[cfg(test)]
mod tests {
    use ty::fun::{
        Val,
        val,
    };
    use ty::nat;
    use super::{
        Append,
        Cons,
        Length,
        Nil,
    };

    #[test]
    fn append() {
        fn aux<A0, A1, A2, A3>() {
            let _: Val<Cons<A0, Cons<A1, Cons<A2, Cons<A3, Nil>>>>> =
                val::<Append, (
                    Cons<A0, Cons<A1, Nil>>,
                    Cons<A2, Cons<A3, Nil>>,
                    )>();
        }
        aux::<(), bool, u8, u16>();
    }

    #[test]
    fn length() {
        fn aux<A0, A1, A2, A3>() {
            let _: Val<nat::_4> =
                val::<Length, (
                    Cons<A0, Cons<A1, Cons<A2, Cons<A3, Nil>>>>,
                    )>();
        }
        aux::<(), bool, u8, u16>();
    }
}
