#[macro_export] macro_rules! ph {
    () => (::std::marker::PhantomData);
    ($A:ty) => (::std::marker::PhantomData::<$A>);
}

#[macro_export] macro_rules! Ph {
    ($A:ty) => (::std::marker::PhantomData<$A>);
}
