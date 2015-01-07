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
impl<RHS: List> fun::Fn<Append, ( Nil, RHS, )> for fun::Call
{
    type T = RHS;
}
impl<H, LHS: List, RHS: List> fun::Fn<Append, ( Cons<H, LHS>, RHS, )> for fun::Call where
    fun::Call: fun::Fn<Append, ( LHS, RHS, )>,
{
    type T = Cons<H, <fun::Call as fun::Fn<Append, ( LHS, RHS, )>>::T>;
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
impl fun::Fn<Length, ( Nil, )> for fun::Call {
    type T = nat::Z;
}
impl<H, T: List> fun::Fn<Length, ( Cons<H, T>, )> for fun::Call where
    fun::Call: fun::Fn<Length, ( T, )>,
{
    type T = nat::S<<fun::Call as fun::Fn<Length, ( T, )>>::T>;
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
