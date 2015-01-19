use hlist::{
    HC,
    HN,
};
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
impl<A: Ty> Sig for Append<A> { type Dom = HC<List<A>, HC<List<A>, HN>>; type Cod = List<A>; }
impl<A: Ty, L1: Tm<List<A>>> FnTm<Append<A>> for HC<Nil, HC<L1, HN>>
{
    type O = L1;
}
impl<A: Ty, H: Tm<A>, L0: Tm<List<A>>, L1: Tm<List<A>>, Rec: Tm<List<A>>> FnTm<Append<A>> for HC<Cons<H, L0>, HC<L1, HN>> where
    HC<L0, HC<L1, HN>>: FnTm<Append<A>, O = Rec>,
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
impl<A: Ty> Sig for Length<A> { type Dom = List<A>; type Cod = nat::peano::Nat; }
impl<A: Ty> FnTm<Length<A>> for Nil {
    type O = nat::peano::Zero;
}
impl<A: Ty, H: Tm<A>, T: Tm<List<A>>, Rec: Tm<nat::peano::Nat>> FnTm<Length<A>> for Cons<H, T> where
    T: FnTm<Length<A>, O = Rec>,
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
impl<A: Ty> Sig for At<A> { type Dom = HC<List<A>, HC<nat::peano::Nat, HN>>; type Cod = A; }
impl<A: Ty, H: Tm<A>, T: Tm<List<A>>> FnTm<At<A>> for HC<Cons<H, T>, HC<nat::peano::Zero, HN>> {
    type O = H;
}
impl<A: Ty, H: Tm<A>, T: Tm<List<A>>, N: Tm<nat::peano::Nat>, Rec: Tm<A>> FnTm<At<A>> for HC<Cons<H, T>, HC<nat::peano::Succ<N>, HN>> where
    HC<T, HC<N, HN>>: FnTm<At<A>, O = Rec>,
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
impl<A: Ty> Sig for ReplaceAt<A> { type Dom = HC<List<A>, HC<nat::peano::Nat, HC<A, HN>>>; type Cod = List<A>; }
impl<A: Ty, H: Tm<A>, T: Tm<List<A>>, X: Tm<A>> FnTm<ReplaceAt<A>> for HC<Cons<H, T>, HC<nat::peano::Zero, HC<X, HN>>> {
    type O = Cons<X, T>;
}
impl<A: Ty, H: Tm<A>, T: Tm<List<A>>, N: Tm<nat::peano::Nat>, X: Tm<A>, Rec: Tm<List<A>>> FnTm<ReplaceAt<A>> for HC<Cons<H, T>, HC<nat::peano::Succ<N>, HC<X, HN>>> where
    HC<T, HC<N, HC<X, HN>>>: FnTm<ReplaceAt<A>, O = Rec>,
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
        let _: Wit<LC<HN, LC<HN, LC<HN, LC<HN, LN>>>>> =
            wit::<Append<HN>,
                HC<LC<HN, LC<HN, LN>>,
                HC<LC<HN, LC<HN, LN>>,
                HN>>>();
    }

    #[test]
    fn length() {
        let _: Wit<_4n> =
            wit::<Length<HN>, LC<HN, LC<HN, LC<HN, LC<HN, LN>>>>>();
    }

    #[test]
    fn at() {
        let _: Wit<_2n> =
            wit::<At<Nat>,
                HC<LC<_4n, LC<_2n, LN>>,
                HC<_1n,
                HN>>>();
    }

    #[test]
    fn at_star() {
        let _: Wit<Rust<Option<u8>>> =
            wit::<At<Star>,
                HC<LC<Rust<Option<u8>>, LC<Rust<Vec<HN>>, LN>>,
                HC<_0n,
                HN>>>();
    }

    #[test]
    fn replace_at() {
        let _: Wit<LC<_4n, LC<_5n, LN>>> =
            wit::<ReplaceAt<Nat>,
                HC<LC<_4n, LC<_2n, LN>>,
                HC<_1n,
                HC<_5n,
                HN>>>>();
    }
}
