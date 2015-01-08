use ty::{
    Eq,
};

/// A `Singleton` trait enforces the constraint that `T` must equal `Self`
pub trait Singleton<T> where Self: Eq<T> {}

impl<T> Singleton<T> for T {}
