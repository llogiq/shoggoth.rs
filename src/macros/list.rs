#[macro_export] macro_rules! List {
    {} => {
        $crate::data::list::Nil
    };
    { $head:ty } => {
        $crate::data::list::Cons<$head, $crate::data::list::Nil>
    };
    { $head:ty, $($rest:ty),+ } => {
        $crate::data::list::Cons<$head, List![$($rest),+]>
    };
    { $($head:ty),+ : $tail:ty } => {
        ListTail!({ $tail } $($head),+)
    };
}

macro_rules! ListTail {
    { { $tail:ty } } => {
        $tail
    };
    { { $tail:ty } $head:ty } => {
        $crate::data::list::Cons<$head, $tail>
    };
    { { $tail:tt } $head:ty, $($rest:tt),+ } => {
        $crate::data::list::Cons<$head, ListTail!({ $tail } $($rest),+)>
    };
}

#[macro_export] macro_rules! list {
    {} => {
        $crate::data::list::Nil
    };
    {=> $($elem:tt)+ } => {
        list_pat!($($elem)+)
    };
    { $head:expr } => {
        $crate::data::list::Cons($head, $crate::data::list::Nil)
    };
    { $head:expr, $($tail:expr),+ } => {
        $crate::data::list::Cons($head, list!($($tail),+))
    };
    { $($head:expr),+ : $tail:expr } => {
        list_expr_tail!({ $tail } $($head),+)
    };
}

macro_rules! list_expr_tail {
    { { $tail:expr } } => {
        $tail
    };
    { { $tail:expr } $head:expr } => {
        $crate::data::list::Cons($head, $tail)
    };
    { { $tail:tt } $head:expr, $($rest:tt),+ } => {
        $crate::data::list::Cons($head, list_expr_tail!({ $tail } $($rest),+))
    };
}

macro_rules! list_pat {
    {} => {
        $crate::data::list::Nil
    };
    { $head:pat } => {
        $crate::data::list::Cons($head, $crate::data::list::Nil)
    };
    { $head:pat, $($rest:tt),+ } => {
        $crate::data::list::Cons($head, list_pat!($($rest),+))
    };
    { $($head:pat),+ : $tail:pat } => {
        list_pat_tail!({ $tail } $($head),+)
    };
}

macro_rules! list_pat_tail {
    { { $tail:pat } } => {
        $tail
    };
    { { $tail:pat } $head:pat } => {
        $crate::data::list::Cons($head, $tail)
    };
    { { $tail:tt } $head:pat, $($rest:tt),+ } => {
        $crate::data::list::Cons($head, list_pat_tail!({ $tail } $($rest),+))
    };
}
