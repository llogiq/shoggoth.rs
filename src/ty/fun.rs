use ty;
use singleton::{
    Singleton,
};

pub trait Fn<This, In>: Singleton<This> {
    type Out;
}

pub trait DepFn<This, In>: self::Fn<This, In> {
    #[inline]
    fn call<X: ty::eq::Eq<Self>>(arg: In) -> <Self as self::Fn<This, In>>::Out;
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

impl Val<()> {
    #[inline]
    pub fn val<F: self::Fn<F, X>, X>() -> Val<<F as self::Fn<F, X>>::Out> { Val(()) }
}

