use ty::{
    FnTm,
    Sig,
    Tm,
    Ty,
};
use ty::nat;

/// Type-level list
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
pub enum Cons<H, T> {}
impl<A: Ty, H: Tm<A>, T: Tm<List<A>>> Tm<List<A>> for Cons<H, T> {}

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
impl<A: Ty> Sig for Append<A> { type Dom = (List<A>, List<A>,); type Cod = List<A>; }
impl<A: Ty, L1: Tm<List<A>>> FnTm<Append<A>> for (Nil, L1,)
{
    type O = L1;
}
impl<A: Ty, H: Tm<A>, L0: Tm<List<A>>, L1: Tm<List<A>>, Rec: Tm<List<A>>> FnTm<Append<A>> for (Cons<H, L0>, L1,) where
    (L0, L1,): FnTm<Append<A>, O = Rec>,
{
    type O = Cons<H, Rec>;
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
impl<A: Ty> Sig for Length<A> { type Dom = (List<A>,); type Cod = nat::peano::Nat; }
impl<A: Ty> FnTm<Length<A>> for (Nil,) {
    type O = nat::peano::Zero;
}
impl<A: Ty, H: Tm<A>, T: Tm<List<A>>, Rec: Tm<nat::peano::Nat>> FnTm<Length<A>> for (Cons<H, T>,) where
    (T,): FnTm<Length<A>, O = Rec>,
{
    type O = nat::peano::Succ<Rec>;
}

/// Type-level function for list look up at index
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum At<A: Ty> {}
impl<A: Ty> Sig for At<A> { type Dom = (List<A>, nat::peano::Nat,); type Cod = A; }
impl<A: Ty, H: Tm<A>, T: Tm<List<A>>> FnTm<At<A>> for (Cons<H, T>, nat::peano::Zero,) {
    type O = H;
}
impl<A: Ty, H: Tm<A>, T: Tm<List<A>>, N: Tm<nat::peano::Nat>, Rec: Tm<A>> FnTm<At<A>> for (Cons<H, T>, nat::peano::Succ<N>,) where
    (T, N,): FnTm<At<A>, O = Rec>,
{
    type O = Rec;
}

/// Type-level function for list replace at index
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum ReplaceAt<A: Ty> {}
impl<A: Ty> Sig for ReplaceAt<A> { type Dom = (List<A>, nat::peano::Nat, A); type Cod = List<A>; }
impl<A: Ty, H: Tm<A>, T: Tm<List<A>>, X: Tm<A>> FnTm<ReplaceAt<A>> for (Cons<H, T>, nat::peano::Zero, X,) {
    type O = Cons<X, T>;
}
impl<A: Ty, H: Tm<A>, T: Tm<List<A>>, N: Tm<nat::peano::Nat>, X: Tm<A>, Rec: Tm<List<A>>> FnTm<ReplaceAt<A>> for (Cons<H, T>, nat::peano::Succ<N>, X,) where
    (T, N, X,): FnTm<ReplaceAt<A>, O = Rec>,
{
    type O = Cons<H, Rec>;
}

#[cfg(test)]
mod tests {
    use ty::{
        Rust,
        Star,
    };
    use ty::list::*;
    use ty::literal::*;
    use ty::nat::peano::*;
    use ty::wit::*;

    #[test]
    fn append() {
        let _: Wit<LC<(), LC<(), LC<(), LC<(), LN>>>>> =
            wit::<Append<_>, (
                LC<(), LC<(), LN>>,
                LC<(), LC<(), LN>>,
                )>();
    }

    #[test]
    fn length() {
        let _: Wit<_4n> =
            wit::<Length<_>, (
                LC<(), LC<(), LC<(), LC<(), LN>>>>,
                )>();
    }

    #[test]
    fn at() {
        let _: Wit<_2n> =
            wit::<At<Nat>, (
                LC<_4n, LC<_2n, LN>>,
                _1n,
                )>();
    }

    #[test]
    fn at_star() {
        let _: Wit<Rust<Option<u8>>> =
            wit::<At<Star>, (
                LC<Rust<Option<u8>>, LC<Rust<Vec<()>>, LN>>,
                _0n,
                )>();
    }

    #[test]
    fn replace_at() {
        let _: Wit<LC<_4n, LC<_5n, LN>>> =
            wit::<ReplaceAt<Nat>, (
                LC<_4n, LC<_2n, LN>>,
                _1n,
                _5n,
                )>();
    }
}
