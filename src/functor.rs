use hkt;
use ty;

/// Functors using the HKT encoding
pub trait Functor<T>: hkt::Arg + hkt::Tag<O = T> {
    type X: hkt::Ins<T> + ty::Eq<hkt::Un<Self>>;
    #[inline]
    fn fmap<B, F>(self, f: F) -> hkt::Ap<T, B> where F: Fn(Self::X) -> B;
}

/// Type-level function for "unapplying" a functor, returning the parameter
pub type Un<T, TX: Functor<T>> = <TX as Functor<T>>::X;

impl<A> Functor<hkt::tag::Option> for Option<A> {
    type X = A;
    #[inline]
    fn fmap<B, F>(self, f: F) -> Option<B> where F: Fn(A) -> B
    {
        self.map(f)
    }
}

impl<A> Functor<hkt::tag::Vec> for Vec<A> {
    type X = A;
    #[inline]
    fn fmap<B, F>(self, f: F) -> Vec<B> where F: Fn(A) -> B
    {
        self.into_iter().map(f).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Functor,
        Un,
    };
    use hkt;
    #[test]
    fn fmap_inline() {
        // works pretty well inline
        println!("{}", Some(true).fmap(|&: b| !b).unwrap());
        println!("{}", vec![true, false].fmap(|&: b| !b)[0]);
    }

    #[test]
    fn fmap_generic() {
        // bit more work to use generically; let's use a reference for the hell of it
        fn aux<T, TA, B, Clo>(m: TA, f: &Clo) -> hkt::Ap<T, B> where
            TA: Functor<T>,
        B: hkt::Ins<T>,
        Clo: Fn(Un<T, TA>) -> B
        {
            m.fmap(|&: x| f(x))
        }
        let f = |&: b: bool| !b;
        aux(Some(true), &f);
        aux(vec![true, false], &f);
    }
}
