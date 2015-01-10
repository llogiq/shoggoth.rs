use functor as ftor;
use functor::{
    Functor,
};
use hkt;
use unify;

/// Monads using the HKT encoding
pub trait Monad<T>: Functor<T> {
    #[inline]
    fn point<This>(x: ftor::Un<T, Self>) -> Self where (): unify::BiEq<This, Self>;
    #[inline]
    fn join<This>(m: hkt::Ap<T, Self>) -> Self where (): unify::BiEq<This, Self>;
}

impl<A> Monad<hkt::tag::Option> for Option<A> {
    #[inline]
    fn point<This>(x: A) -> Option<A> { Some(x) }
    #[inline]
    fn join<This>(m: Option<Option<A>>) -> Option<A> {
        match m {
            None => None,
            Some(x) => x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Monad,
    };

    #[test]
    fn point() {
        let _ = Monad::point::<Option<_>>(true);
        let _: Option<_> = Monad::point(true);
    }

    #[test]
    fn join() {
        let _ = Monad::join::<Option<_>>(Some(Some(true)));
        let _: Option<_> = Monad::join(Some(Some(true)));
    }
}
