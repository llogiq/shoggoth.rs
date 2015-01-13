use ty::{
    Tm,
    Ty,
    fun,
    nat,
};

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum List<A: Ty> {}
impl<A: Ty> Ty for List<A> {}

/// Type-level nil list
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Nil {}
impl<A: Ty> Tm<List<A>> for Nil {}

/// Type-level cons list
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Cons<A: Ty, H: Tm<A>, T: Tm<List<A>>> {}
impl<A: Ty, H: Tm<A>, T: Tm<List<A>>> Tm<List<A>> for Cons<A, H, T> {}

/// Type-level function for list append
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Append<A: Ty> {}
impl<A: Ty> fun::Sig for Append<A> { type Dom = (List<A>, List<A>,); type Cod = List<A>; }
impl<A: Ty, N1: Tm<List<A>>> fun::Fn<Append<A>> for (Nil, N1,)
{
    type O = N1;
}
impl<A: Ty, H: Tm<A>, N0: Tm<List<A>>, N1: Tm<List<A>>> fun::Fn<Append<A>> for (Cons<A, H, N0>, N1,) where
    (N0, N1,): fun::Fn<Append<A>>,
    fun::Ap<Append<A>, (N0, N1,)>: Tm<List<A>>,
{
    type O = Cons<A, H, fun::Ap<Append<A>, (N0, N1,)>>;
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
pub enum Length<A: Ty> {}
impl<A: Ty> fun::Sig for Length<A> { type Dom = (List<A>,); type Cod = nat::Nat; }
impl<A: Ty> fun::Fn<Length<A>> for (Nil,) {
    type O = nat::Zero;
}
impl<A: Ty, H: Tm<A>, T: Tm<List<A>>> fun::Fn<Length<A>> for (Cons<A, H, T>,) where
    (T,): fun::Fn<Length<A>>,
    fun::Ap<Length<A>, (T,)>: Tm<nat::Nat>,
{
    type O = nat::Succ<fun::Ap<Length<A>, (T,)>>;
}

#[cfg(test)]
mod tests {
    use ty::list::*;
    use ty::literal::*;
    use ty::val::*;

    #[test]
    fn append() {
        let _: Val<Cons<(), (), Cons<(), (), Cons<(), (), Cons<(), (), Nil>>>>> =
            val::<Append<()>, (
                Cons<(), (), Cons<(), (), Nil>>,
                Cons<(), (), Cons<(), (), Nil>>,
                )>();
    }

    #[test]
    fn length() {
        let _: Val<_4n> =
            val::<Length<()>, (
                Cons<(), (), Cons<(), (), Cons<(), (), Cons<(), (), Nil>>>>,
                )>();
    }
}
