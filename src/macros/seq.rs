#[macro_export] macro_rules! seq_head {
    { $x:ident } => { $x };
    { $x:ident, $($xs:ident),* } => { $x };
}

#[macro_export] macro_rules! seq_tail {
    { $x:ident } => { () };
    { $x:ident, $($xs:ident),* } => { ($($xs,)*) };
}
