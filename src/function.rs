use equality::{
    Is,
};
use singleton::{
    Singleton,
};

pub trait TFn<This, In>: Singleton<This> {
    type Out;
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
pub struct TVal<X>(());

impl TVal<()> {
    pub fn val<F: TFn<F, X>, X>() -> TVal<<F as TFn<F, X>>::Out> { TVal(()) }
}

pub trait DFn<This, In>: TFn<This, In> {
    fn call<X: Is<Self>>(arg: In) -> <Self as TFn<This, In>>::Out;
}


