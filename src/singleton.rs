use ty::{
    Eq,
};

pub trait Singleton<T> where Self: Eq<T> {}
impl<T> Singleton<T> for T {}
