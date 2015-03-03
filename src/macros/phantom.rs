#[macro_export] macro_rules! ph {
    {} => { ::std::marker::PhantomData };
    { $T:path } => { $T(::std::marker::PhantomData) };
}

#[macro_export] macro_rules! Ph {
    { $A:tt } => { ::std::marker::PhantomData<$A> };
}
