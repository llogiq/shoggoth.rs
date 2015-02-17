#[macro_export] macro_rules! List {
    {} => { $crate::list::Nil };
    { $head:ty } => { $crate::list::Cons<$head, $crate::list::Nil> };
    { $head:ty, $($tail:ty),* } => { $crate::list::Cons<$head, List!($($tail),*)> };
}

#[macro_export] macro_rules! list {
    {} => { $crate::list::Nil };
    { $head:expr } => { $crate::list::Cons($head, $crate::list::Nil) };
    { $head:expr, $($tail:expr),* } => { $crate::list::Cons($head, list!($($tail),*)) };
}

#[macro_export] macro_rules! list_match {
    {} => { $crate::list::Nil };
    { $head:ident } => { $crate::list::Cons($head, $crate::list::Nil) };
    { $head:ident, $($tail:ident),* } => { $crate::list::Cons($head, list_match!($($tail),*)) };
}
