use ty;

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
impl<RHS: List> ty::fun::Fn<(Nil, RHS,)> for Append
{
    type O = RHS;
}
impl<H, LHS: List, RHS: List> ty::fun::Fn<(Cons<H, LHS>, RHS,)> for Append where
    Append: ty::fun::Fn<(LHS, RHS,)>,
{
    type O = Cons<H, ty::fun::Ap<Append, (LHS, RHS,)>>;
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
impl ty::fun::Fn<(Nil,)> for Length {
    type O = ty::nat::Zero;
}
impl<H, T: List> ty::fun::Fn<(Cons<H, T>,)> for Length where
    Length: ty::fun::Fn<(T,)>,
{
    type O = ty::nat::Succ<ty::fun::Ap<Length, (T,)>>;
}

#[cfg(test)]
mod tests {
    use ty::list::*;
    use ty::literal::*;
    use ty::val::*;

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
            let _: Val<_4n> =
                val::<Length, (
                    Cons<A0, Cons<A1, Cons<A2, Cons<A3, Nil>>>>,
                    )>();
        }
        aux::<(), bool, u8, u16>();
    }
}
