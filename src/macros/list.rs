#[macro_export] macro_rules! List {
    {} => {
        $crate::products::list::Nil
    };
    { $head:ty } => {
        $crate::products::list::Cons<$head, $crate::products::list::Nil>
    };
    { $head:ty, $($tail:ty),* } => {
        $crate::products::list::Cons<$head, List!($($tail),*)>
    };
}

#[macro_export] macro_rules! list {
    {} => {
        $crate::products::list::Nil
    };
    {=> $($elem:tt),+ } => {
        list_pat!($($elem),+)
    };
    { $head:expr, $($tail:expr),* } => {
        $crate::products::list::Cons($head, list!($($tail),*))
    };
    { $head:expr } => {
        $crate::products::list::Cons($head, $crate::products::list::Nil)
    };
}

macro_rules! list_pat {
    {} => {
        $crate::products::list::Nil
    };
    { $head:pat, $($tail:tt),* } => {
        $crate::products::list::Cons($head, list_pat!($($tail),*))
    };
    { $head:pat } => {
        $crate::products::list::Cons($head, $crate::products::list::Nil)
    };
}

