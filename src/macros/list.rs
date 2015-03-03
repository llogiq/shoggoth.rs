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
    { $($lhs:ty),+ : $ret:ty } => {
        ListRet!({ $ret } $($lhs),+)
    };
}
macro_rules! ListRet {
    { { $ret:ty } } => {
        $ret
    };
    { { $ret:ty } $lhs:ty } => {
        $crate::list::Cons<$lhs, $ret>
    };
    { { $ret:tt } $lhs:ty, $($rest:tt),+ } => {
        $crate::list::Cons<$lhs, ListRet!({ $ret } $($rest),+)>
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
    { $($lhs:expr),+ : $ret:expr } => {
        list_expr_ret!({ $ret } $($lhs),+)
    };
}

macro_rules! list_expr_ret {
    { { $ret:expr } } => {
        $ret
    };
    { { $ret:expr } $lhs:expr } => {
        $crate::list::Cons($lhs, $ret)
    };
    { { $ret:tt } $lhs:expr, $($rest:tt),+ } => {
        $crate::list::Cons($lhs, list_expr_ret!({ $ret } $($rest),+))
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
    { $($lhs:pat),+ : $ret:pat } => {
        list_pat_ret!({ $ret } $($lhs),+)
    };
}

macro_rules! list_pat_ret {
    { { $ret:pat } } => {
        $ret
    };
    { { $ret:pat } $lhs:pat } => {
        $crate::list::Cons($lhs, $ret)
    };
    { { $ret:tt } $lhs:pat, $($rest:tt),+ } => {
        $crate::list::Cons($lhs, list_pat_ret!({ $ret } $($rest),+))
    };
}
