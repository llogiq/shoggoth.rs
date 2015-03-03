#[macro_export] macro_rules! O {
    { $X:ty , [ $($Xs:tt)* ] } => { O<$X, List![$($Xs)*]> };
    { $X:ty , $Xs:ty } => { O<$X, $Xs> };
}

#[macro_export] macro_rules! o {
    {} => { ph!(O) };
    { $X:ty , [ $($Xs:tt)* ] } => { ph!(O<$X, List![$($Xs)*]>) };
    { $X:ty , $Xs:ty } => { ph!(O<$X, $Xs>) }
}

#[macro_export] macro_rules! I {
    { $X:ty , [ $($Xs:tt)* ] } => { I<$X, List![$($Xs)*]> };
    { $X:ty , $Xs:ty } => { I<$X, $Xs> };
}

#[macro_export] macro_rules! i {
    {} => { ph!(I) };
    { $X:ty , [ $($Xs:tt)* ] } => { ph!(I<$X, List![$($Xs)*]>) };
    { $X:ty , $Xs:ty } => { ph!(I<$X, $Xs>) };
}
