#[macro_export] macro_rules! ty {
    {
        fam $($rest:tt)*
    } => {
        fam_parse_init! {
            { $($rest)* }
        }
    };
}

macro_rules! fam_parse_init {
    {
        { $($rest:tt)* }
    } => {
        fam_parse_fun_type! {
            { $($rest)* }
        }
    };
}

macro_rules! fam_parse_fun_type {
    {
        { :[ $($fun_type:tt)+ ] $($rest:tt)* }
    } => {
        fam_parse_fun_body! {
            { $($rest)* }
            { $($fun_type)+ }
        }
    };
}

macro_rules! fam_parse_fun_body {
    {
        { =[ $($fun_body:tt)+ ] $($rest:tt)* }
        { $($fun_type:tt)* }
    } => {
        fam_parse_let! {
            { $($rest)* }
            { $($fun_type)* }
            { $($fun_body)+ }
        }
    };
}

macro_rules! fam_parse_let {
    {
        { let { $($equations:tt)* } $($rest:tt)* }
        { $($fun_type:tt)* }
        { $($fun_body:tt)* }
    } => {
        fam_parse_for! {
            { $($rest)* }
            { $($fun_type)* }
            { $($fun_body)* }
            { $($equations)* }
        }
    };
    {
        { $($rest:tt)* }
        { $($fun_type:tt)* }
        { $($fun_body:tt)* }
    } => {
        fam_parse_for! {
            { $($rest)* }
            { $($fun_type)* }
            { $($fun_body)* }
            {}
        }
    };
}

macro_rules! fam_parse_for {
    {
        { for { $($variables:tt)* } $($rest:tt)* }
        { $($fun_type:tt)* }
        { $($fun_body:tt)* }
        { $($equations:tt)* }
    } => {
        fam_parse_where! {
            { $($rest)* }
            { $($fun_type)* }
            { $($fun_body)* }
            { $($equations)* }
            { $($variables)* }
        }
    };
    {
        { $($rest:tt)* }
        { $($fun_type:tt)* }
        { $($fun_body:tt)* }
        { $($equations:tt)* }
    } => {
        fam_parse_where! {
            { $($rest)* }
            { $($fun_type)* }
            { $($fun_body)* }
            { $($equations)* }
            {}
        }
    };
}

macro_rules! fam_parse_where {
    {
        { where { $($constraints:tt)* } $($rest:tt)* }
        { $($fun_type:tt)* }
        { $($fun_body:tt)* }
        { $($equations:tt)* }
        { $($variables:tt)* }
    } => {
        fam_parse! {
            { $($rest)* }
            { $($fun_type)+ }
            { $($fun_body)* }
            { $($equations)* }
            { $($variables)* }
            { $($constraints)* }
        }
    };
    {
        { $($rest:tt)* }
        { $($fun_type:tt)* }
        { $($fun_body:tt)* }
        { $($equations:tt)* }
        { $($variables:tt)* }
    } => {
        fam_parse! {
            { $($rest)* }
            { $($fun_type)* }
            { $($fun_body)* }
            { $($equations)* }
            { $($variables)* }
            {}
        }
    };
}

macro_rules! fam_parse {
    {
        {}
        { $Op:ident $Args:ty  => $Output:ty }
        { $op:ident $args:pat => $output:expr }
        {}
        { $($X:ident),* }
        {}
    } => {
        impl<$($X,)*> Fn<$Args> for $Op {
            type Output = $Output;
            extern "rust-call" fn call(&self, $args : $Args) -> $Output {
                $output
            }
        }
    };
    {
        {}
        { $Op:ident $Args:ty  => $Output:ty }
        { $op:ident $args:pat => $output:expr }
        { $($RecVal:ty = $RecOp:ident $RecArgs:ty,)* }
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
    };
}
