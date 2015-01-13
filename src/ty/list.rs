use ty::{
    Tm,
    Ty,
    fun,
    nat,
};

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
impl<A: Ty> fun::Sig for Append<A> { type Dom = (List<A>, List<A>,); type Cod = List<A>; }
impl<A: Ty, L1: Tm<List<A>>> fun::Fn<Append<A>> for (Nil, L1,)
{
    type O = L1;
}
impl<A: Ty, H: Tm<A>, L0: Tm<List<A>>, L1: Tm<List<A>>> fun::Fn<Append<A>> for (Cons<H, L0>, L1,) where
    (L0, L1,): fun::Fn<Append<A>>,
    fun::Ap<Append<A>, (L0, L1,)>: Tm<List<A>>,
{
    type O = Cons<H, fun::Ap<Append<A>, (L0, L1,)>>;
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
impl<A: Ty, H: Tm<A>, T: Tm<List<A>>> fun::Fn<Length<A>> for (Cons<H, T>,) where
    (T,): fun::Fn<Length<A>>,
    fun::Ap<Length<A>, (T,)>: Tm<nat::Nat>,
{
    type O = nat::Succ<fun::Ap<Length<A>, (T,)>>;
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
impl<A: Ty> fun::Sig for At<A> { type Dom = (List<A>, nat::Nat,); type Cod = A; }
impl<A: Ty, H: Tm<A>, T: Tm<List<A>>> fun::Fn<At<A>> for (Cons<H, T>, nat::Zero,) {
    type O = H;
}
impl<A: Ty, H: Tm<A>, T: Tm<List<A>>, N: Tm<nat::Nat>> fun::Fn<At<A>> for (Cons<H, T>, nat::Succ<N>,) where
    (T, N,): fun::Fn<At<A>>,
    fun::Ap<At<A>, (T, N,)>: Tm<A>,
{
    type O = fun::Ap<At<A>, (T, N,)>;
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
impl<A: Ty> fun::Sig for ReplaceAt<A> { type Dom = (List<A>, nat::Nat, A); type Cod = List<A>; }
impl<A: Ty, H: Tm<A>, T: Tm<List<A>>, X: Tm<A>> fun::Fn<ReplaceAt<A>> for (Cons<H, T>, nat::Zero, X,) {
    type O = Cons<X, T>;
}
impl<A: Ty, H: Tm<A>, T: Tm<List<A>>, N: Tm<nat::Nat>, X: Tm<A>> fun::Fn<ReplaceAt<A>> for (Cons<H, T>, nat::Succ<N>, X,) where
    (T, N, X,): fun::Fn<ReplaceAt<A>>,
    fun::Ap<ReplaceAt<A>, (T, N, X)>: Tm<List<A>>,
{
    type O = Cons<H, fun::Ap<ReplaceAt<A>, (T, N, X)>>;
}

#[cfg(test)]
mod tests {
    use ty::{
        Rust,
        Star,
    };
    use ty::list::*;
    use ty::literal::*;
    use ty::nat::*;
    use ty::val::*;

    #[test]
    fn append() {
        let _: Val<Cl<(), Cl<(), Cl<(), Cl<(), Nl>>>>> =
            val::<Append<_>, (
                Cl<(), Cl<(), Nl>>,
                Cl<(), Cl<(), Nl>>,
                )>();
    }

    #[test]
    fn length() {
        let _: Val<_4n> =
            val::<Length<_>, (
                Cl<(), Cl<(), Cl<(), Cl<(), Nl>>>>,
                )>();
    }

    #[test]
    fn at() {
        let _: Val<_2n> =
            val::<At<Nat>, (
                Cl<_4n, Cl<_2n, Nl>>,
                _1n,
                )>();
    }

    #[test]
    fn at_star() {
        let _: Val<Rust<Option<u8>>> =
            val::<At<Star>, (
                Cl<Rust<Option<u8>>, Cl<Rust<Vec<()>>, Nl>>,
                _0n,
                )>();
    }

    #[test]
    fn replace_at() {
        let _: Val<Cl<_4n, Cl<_5n, Nl>>> =
            val::<ReplaceAt<Nat>, (
                Cl<_4n, Cl<_2n, Nl>>,
                _1n,
                _5n,
                )>();
    }
}
