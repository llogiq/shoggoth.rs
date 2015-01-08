use ty;

/// A `Singleton` trait enforces the constraint that `T` must equal `Self`
pub trait Singleton<T> where Self: ty::Eq<T> {}

impl<T> Singleton<T> for T {}
