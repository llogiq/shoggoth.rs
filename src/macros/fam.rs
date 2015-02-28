#[macro_export] macro_rules! ty {
    {
        fam $($rest:tt)*
    } => {
        parse_fam_init! {
            { $($rest)* }
        }
    };
}

macro_rules! parse_fam_init {
    {
        { $($rest:tt)* }
    } => {
        parse_fam_fun_type! {
            { $($rest)* }
        }
    };
}

macro_rules! parse_fam_fun_type {
    {
        { :[ $($fun_type:tt)+ ] $($rest:tt)* }
    } => {
        parse_fam_fun_body! {
            { $($rest)* }
            { $($fun_type)+ }
        }
    };
}

macro_rules! parse_fam_fun_body {
    {
        { =[ $($fun_body:tt)+ ] $($rest:tt)* }
        { $($fun_type:tt)* }
    } => {
        parse_fam_let! {
            { $($rest)* }
            { $($fun_type)+ }
            { $($fun_body)* }
        }
    };
}

macro_rules! parse_fam_let {
    {
        { let { $($equations:tt)* } $($rest:tt)* }
        { $($fun_type:tt)* }
        { $($fun_body:tt)* }
    } => {
        parse_fam_for! {
            { $($rest)* }
            { $($fun_type)+ }
            { $($fun_body)* }
            { $($equations)* }
        }
    };
    {
        { $($rest:tt)* }
        { $($fun_type:tt)* }
        { $($fun_body:tt)* }
    } => {
        parse_fam_for! {
            { $($rest)* }
            { $($fun_type)+ }
            { $($fun_body)* }
            {}
        }
    };
}

macro_rules! parse_fam_for {
    {
        { for { $($variables:tt)* } $($rest:tt)* }
        { $($fun_type:tt)* }
        { $($fun_body:tt)* }
        { $($equations:tt)* }
    } => {
        parse_fam_where! {
            { $($rest)* }
            { $($fun_type)+ }
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
        parse_fam_where! {
            { $($rest)* }
            { $($fun_type)+ }
            { $($fun_body)* }
            { $($equations)* }
            {}
        }
    };
}

macro_rules! parse_fam_where {
    {
        { where { $($constraints:tt)* } $($rest:tt)* }
        { $($fun_type:tt)* }
        { $($fun_body:tt)* }
        { $($equations:tt)* }
        { $($variables:tt)* }
    } => {
        parse_fam! {
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
        parse_fam! {
            { $($rest)* }
            { $($fun_type)+ }
            { $($fun_body)* }
            { $($equations)* }
            { $($variables)* }
            {}
        }
    };
}

macro_rules! parse_fam {
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
