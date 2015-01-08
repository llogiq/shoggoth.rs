use ty;

pub trait Fn<I> { type O; }

pub type Ap<F: Fn<I>, I> = <F as Fn<I>>::O;

pub trait DepFn<I>: self::Fn<I> {
    #[inline]
    fn call<X: ty::eq::Eq<Self>>(arg: I) -> Ap<Self, I>;
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Rand)]
#[derive(Show)]
pub struct Val<X>(());

#[inline]
pub fn val<F: Fn<I>, I>() -> Val<Ap<F, I>> { Val(()) }
