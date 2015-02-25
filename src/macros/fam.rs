#[macro_export] macro_rules! ty {
    // - let; - [.]; - [:];
    {
        $(#[ $attr:meta ])*
        // fam $Op:ident ($($Args:ty),*) -> $Output:ty {
        fam $Op:ident $Args:ty  => $Output:ty {
            $op:ident $args:pat => $output:expr
        }
    } => {
        impl Fn<$Args> for $Op {
            type Output = $Output;
            $(#[$attr])*
            extern "rust-call" fn call(&self, $args : $Args) -> $Output {
                $output
            }
        }
    };

    // - let; + [.]; - [:]
    {
        $(#[ $attr:meta ])*
        fam $Op:ident $Args:ty  => $Output:ty {
            $op:ident $args:pat => $output:expr
        } for .[ $($A:ident),* ]
    } => {
        ty! {
            $(#[$attr])*
            fam $Op $Args => $Output {
                $op $args => $output
            } for .[ $($A),* ] :[]
        }
    };

    // - let; - [.]; + [:]
    {
        $(#[ $attr:meta ])*
        fam $Op:ident $Args:ty  => $Output:ty {
            $op:ident $args:pat => $output:expr
        } for :[ $($B:ident : $T:ident),* ]
    } => {
        ty! {
            $(#[$attr])*
            fam $Op $Args => $Output {
                $op $args => $output
            } for .[] :[ $($B : $T),* ]
        }
    };

    // - let; + [.]; + [:]
    {
        $(#[ $attr:meta ])*
        fam $Op:ident $Args:ty  => $Output:ty {
            $op:ident $args:pat => $output:expr
        } for .[ $($A:ident),* ] :[ $($B:ident : $T:ident),* ]
    } => {
        impl<$($A,)* $($B : $T,)*> Fn<$Args> for $Op {
            type Output = $Output;
            $(#[$attr])*
            extern "rust-call" fn call(&self, $args : $Args) -> $Output {
                $output
            }
        }
    };

    // + let; - [.]; - [:]
    {
        $(#[ $attr:meta ])*
        fam $Op:ident $Args:ty  => $Output:ty {
            $op:ident $args:pat => $output:expr
        } let {
            $($E:ty = $F:ident $Xs:ty ,)*
        }
    } => {
        impl Fn<$Args> for $Op where
            $($F : Fn<$Xs, Output = $E>),*
        {
            type Output = $Output;
            $(#[$attr])*
            extern "rust-call" fn call(&self, $args : $Args) -> $Output {
                $output
            }
        }
    };

    // + let; + [.]; - [:]
    {
        $(#[ $attr:meta ])*
        fam $Op:ident $Args:ty  => $Output:ty {
            $op:ident $args:pat => $output:expr
        } let {
            $($E:ty = $F:ident $Xs:ty ,)*
        } for .[ $($A:ident),* ]
    } => {
        ty! {
            $(#[$attr])*
            fam $Op $Args => $Output {
                $op $args => $output
            } let {
                $($E = $F $Xs,)*
            } for .[ $($A),* ] :[]
        }
    };

    // + let; - [.]; + [:]
    {
        $(#[ $attr:meta ])*
        fam $Op:ident $Args:ty  => $Output:ty {
            $op:ident $args:pat => $output:expr
        } let {
            $($E:ty = $F:ident $Xs:ty ,)*
        } for :[ $($B:ident : $T:ident),* ]
    } => {
        ty! {
            $(#[$attr])*
            fam $Op $Args => $Output {
                $op $args => $output
            } let {
                $($E = $F $Xs,)*
            } for .[] :[ $($B : $T),* ]
        }
    };

    // + let; + [.]; + [:]
    {
        $(#[ $attr:meta ])*
        fam $Op:ident $Args:ty  => $Output:ty {
            $op:ident $args:pat => $output:expr
        } let {
            $($E:ty = $F:ident $Xs:ty ,)*
        } for .[ $($A:ident),* ] :[ $($B:ident : $T:ident),* ]
    } => {
        impl<$($A,)* $($B : $T,)*> Fn<$Args> for $Op where
            $($F : Fn<$Xs, Output = $E>),*
        {
            type Output = $Output;
            $(#[$attr])*
            extern "rust-call" fn call(&self, $args : $Args) -> $Output {
                $output
            }
        }
    };
}
