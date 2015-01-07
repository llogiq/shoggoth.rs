use ty::eq::{
    Is,
};

pub trait Singleton<T> where Self: Is<T> {}
impl<T> Singleton<T> for T {}
