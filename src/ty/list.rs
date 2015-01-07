use ty::fun;
use ty::nat;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Nil {}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Cons<H, T: List> {}

pub trait List {}
impl List for Nil {}
impl<H, T: List> List for Cons<H, T> {}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Append {}
impl<RHS: List> fun::Fn<Append, ( Nil, RHS, )> for Append
{
    type Out = RHS;
}
impl<H, LHS: List, RHS: List> fun::Fn<Append, ( Cons<H, LHS>, RHS, )> for Append where
    Append: fun::Fn<Append, ( LHS, RHS, )>,
{
    type Out = Cons<H, <Append as fun::Fn<Append, ( LHS, RHS, )>>::Out>;
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Length {}
impl fun::Fn<Length, ( Nil, )> for Length {
    type Out = nat::Z;
}
impl<H, T: List> fun::Fn<Length, ( Cons<H, T>, )> for Length where
    Length: fun::Fn<Length, ( T, )>,
{
    type Out = nat::S<<Length as fun::Fn<Length, ( T, )>>::Out>;
}

#[cfg(test)]
mod tests {
    use ty::fun;
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
            let _: fun::Val< Cons<A0, Cons<A1, Cons<A2, Cons<A3, Nil>>>> > =
                fun::Val::val::<Append, (
                    Cons<A0, Cons<A1, Nil>>,
                    Cons<A2, Cons<A3, Nil>>,
                    )>();
        }
        aux::<(), bool, u8, u16>();
    }

    #[test]
    fn length() {
        fn aux<A0, A1, A2, A3>() {
            let _: fun::Val< nat::N04 > =
                fun::Val::val::<Length, (
                    Cons<A0, Cons<A1, Cons<A2, Cons<A3, Nil>>>>,
                    )>();
        }
        aux::<(), bool, u8, u16>();
    }
}
