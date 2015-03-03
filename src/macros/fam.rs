macro_rules! ty {
    { fam $Op:ident :: Fn ( $($dom:tt),* ) -> $cod:tt where $($rest:tt)+ } => {
        $(fam_kind! { $dom })* fam_kind! { $cod }
        #[doc(hidden)] pub struct $Op;
        fam_impls! { { $($rest)+ } { $Op } }
    };
}

macro_rules! fam_kind {
    { type } => {};
    { $T:ident } => {};
}

macro_rules! fam_impls {
    {
        { }
        { $Op:tt } } => {};
    {
        { $($rest:tt)+ }
        { $Op:tt }
    } => {
        fam_parse_fun_type! {
            { $($rest)+ }
            { $Op }
        }
    };
}

macro_rules! fam_parse_fun_type {
    {
        { $Args:ty => $Output:ty = $($rest:tt)* }
        { $Op:tt }
    } => {
        fam_parse_fun_body! {
            { $($rest)* }
            { $Op }
            { $Args => $Output }
        }
    };
}

macro_rules! fam_parse_fun_body {
    {
        { { .. $($fun_body:tt)+ } $($rest:tt)* }
        { $Op:tt }
        { $($fun_type:tt)* }
    } => {
        fam_parse_let! {
            { $($rest)* }
            { $Op }
            { $($fun_type)* }
            { _ => $($fun_body)+ }
        }
    };
    {
        { { $($fun_body:tt)+ } $($rest:tt)* }
        { $Op:tt }
        { $($fun_type:tt)* }
    } => {
        fam_parse_let! {
            { $($rest)* }
            { $Op }
            { $($fun_type)* }
            { $($fun_body)+ }
        }
    };
}

macro_rules! fam_parse_let {
    {
        { let { $($equations:tt)* } $($rest:tt)* }
        { $Op:tt }
        { $($fun_type:tt)* }
        { $($fun_body:tt)* }
    } => {
        fam_parse_for! {
            { $($rest)* }
            { $Op }
            { $($fun_type)* }
            { $($fun_body)* }
            { $($equations)* }
        }
    };
    {
        { $($rest:tt)* }
        { $Op:tt }
        { $($fun_type:tt)* }
        { $($fun_body:tt)* }
    } => {
        fam_parse_for! {
            { $($rest)* }
            { $Op }
            { $($fun_type)* }
            { $($fun_body)* }
            { }
        }
    };
}

macro_rules! fam_parse_for {
    {
        { for { $($variables:tt)* } $($rest:tt)* }
        { $Op:tt }
        { $($fun_type:tt)* }
        { $($fun_body:tt)* }
        { $($equations:tt)* }
    } => {
        fam_parse_where! {
            { $($rest)* }
            { $Op }
            { $($fun_type)* }
            { $($fun_body)* }
            { $($equations)* }
            { $($variables)* }
        }
    };
    {
        { $($rest:tt)* }
        { $Op:tt }
        { $($fun_type:tt)* }
        { $($fun_body:tt)* }
        { $($equations:tt)* }
    } => {
        fam_parse_where! {
            { $($rest)* }
            { $Op }
            { $($fun_type)* }
            { $($fun_body)* }
            { $($equations)* }
            { }
        }
    };
}

macro_rules! fam_parse_where {
    {
        { where { $($constraints:tt)* } $($rest:tt)* }
        { $Op:tt }
        { $($fun_type:tt)* }
        { $($fun_body:tt)* }
        { $($equations:tt)* }
        { $($variables:tt)* }
    } => {
        fam_parse! {
            { $($rest)* }
            { $Op }
            { $($fun_type)+ }
            { $($fun_body)* }
            { $($equations)* }
            { $($variables)* }
            { $($constraints)* }
        }
    };
    {
        { $($rest:tt)* }
        { $Op:tt }
        { $($fun_type:tt)* }
        { $($fun_body:tt)* }
        { $($equations:tt)* }
        { $($variables:tt)* }
    } => {
        fam_parse! {
            { $($rest)* }
            { $Op }
            { $($fun_type)* }
            { $($fun_body)* }
            { $($equations)* }
            { $($variables)* }
            { }
        }
    };
}

macro_rules! fam_parse {
    {
        { $($rest:tt)* }
        { $Op:ident }
        { $Args:ty  => $Output:ty }
        { $args:pat => $output:expr }
        { }
        { $($X:ident),* }
        { }
    } => {
        impl<$($X,)*> Fn<$Args> for $Op {
            type Output = $Output;
            extern "rust-call" fn call(&self, $args : $Args) -> $Output {
                $output
            }
        }
        fam_impls! { { $($rest)* } { $Op } }
    };
    {
        { $($rest:tt)* }
        { $Op:ident }
        { $Args:ty  => $Output:ty }
        { $args:pat => $output:expr }
        { $($RecVal:ty = $RecOp:ident $RecArgs:ty),* }
        { $($X:ident),* }
        { $($T:ty : $C:ident),* }
    } => {
        impl<$($X,)*> Fn<$Args> for $Op where
            $($RecOp : Fn<$RecArgs, Output = $RecVal>,)*
            $($T : $C,)*
        {
            type Output = $Output;
            extern "rust-call" fn call(&self, $args : $Args) -> $Output {
                $output
            }
        }
        fam_impls! { { $($rest)* } { $Op } }
    };
}
