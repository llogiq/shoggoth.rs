use ty;
use singleton::{
    Singleton,
};

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Call {}

pub trait Fn<This, In>: Singleton<Call> {
    type T;
}

pub trait DepFn<This, In>: self::Fn<This, In> {
    #[inline]
    fn call<X: ty::eq::Eq<Self>>(arg: In) -> <Self as self::Fn<This, In>>::T;
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
    pub fn val<F, X>() -> Val<<Call as self::Fn<F, X>>::T> where
        Call: self::Fn<F, X>,
    {
        Val(())
    }
}

