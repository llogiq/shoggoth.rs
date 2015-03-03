#[macro_export] macro_rules! List {
    {} => {
        $crate::list::Nil
    };
    { $lhs:ty } => {
        $crate::list::Cons<$lhs, $crate::list::Nil>
    };
    { $lhs:ty, $($rest:ty),+ } => {
        $crate::list::Cons<$lhs, List![$($rest)+]>
    };
    };
}

#[macro_export] macro_rules! list {
    {} => {
        $crate::list::Nil
    };
    {=> $($elem:tt)+ } => {
        list_pat!($($elem)+)
    };
    { $head:expr } => {
        $crate::list::Cons($head, $crate::list::Nil)
    };
    { $head:expr, $($tail:expr),+ } => {
        $crate::list::Cons($head, list!($($tail),+))
    };
    };
}

macro_rules! list_pat {
    {} => {
        $crate::list::Nil
    };
    { $lhs:pat } => {
        $crate::list::Cons($lhs, $crate::list::Nil)
    };
    { $lhs:pat, $($rest:tt),+ } => {
        $crate::list::Cons($lhs, list_pat!($($rest),+))
    };
    };
}

