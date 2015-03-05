use data::bit::*;
use data::nat::*;

// Head ////////////////////////////////////////////////////////////////////////

ty! {
    fam Head :: Fn(Nat) -> Nat where
        (O!(_1, Xs))        => _1        = { .. _1 } for { Xs }
        (I!(_1, Xs))        => _1        = { .. _1 } for { Xs }
        (O!(O!(X, Xs), Ys)) => O!(X, Xs) = { .. o!() } for { X, Xs, Ys }
        (I!(O!(X, Xs), Ys)) => O!(X, Xs) = { .. o!() } for { X, Xs, Ys }
        (O!(I!(X, Xs), Ys)) => I!(X, Xs) = { .. i!() } for { X, Xs, Ys }
        (I!(I!(X, Xs), Ys)) => I!(X, Xs) = { .. i!() } for { X, Xs, Ys }
}

// Successor ///////////////////////////////////////////////////////////////////

ty! {
    fam Succ :: Fn(Nat) -> Nat where
        (_1,)                  => O!(_1, [])           = { .. o!() }
        (O!(_1, []),)          => I!(_1, [])           = { .. i!() }
        (O!(_1, [X : Xs]),)    => I!(Sx, Xs)           = { .. i!() } let { Sx = Succ(X,) } for { Sx, X, Xs }
        (O!(Z, Xs),)           => I!(_1, [Pz : Xs])    = { .. i!() } let { Pz = Pred(Z,) } for { Pz, Xs, Z }
        (I!(Z, []),)           => O!(Sz, [])           = { .. o!() } let { Sz = Succ(Z,) } for { Sz, Z }
        (I!(Z, [_1]),)         => O!(Z, [_1])          = { .. o!() } for { Z }
        (I!(Z, [_1, Y : Ys]),) => O!(Z, [Sy : Ys])     = { .. o!() } let { Sy = Succ(Y,) } for { Sy, Y, Ys, Z }
        (I!(Z, [X : Xs]),)     => O!(Z, [_1, Px : Xs]) = { .. o!() } let { Px = Pred(X,) } for { Px, X, Xs, Z }
}

// Predecessor /////////////////////////////////////////////////////////////////

ty! {
    fam Pred :: Fn(Nat) -> Nat where
        (O!(_1, []),)          => _1                   = { ..   _1 }
        (O!(Z, []),)           => I!(Pz, [])           = { .. i!() } let { Pz = Pred(Z,) } for { Pz, Z }
        (O!(Z, [_1]),)         => I!(Z, [_1])          = { .. i!() } for { Z }
        (O!(Z, [_1, X : Xs]),) => I!(Z, [Sx : Xs])     = { .. i!() } let { Sx = Succ(X,) } for { Sx, X, Xs, Z }
        (O!(Z, [X : Xs]),)     => I!(Z, [_1, Px : Xs]) = { .. i!() } let { Px = Pred(X,) } for { Px, X, Xs, Z }
        (I!(_1, []),)          => O!(_1, [])           = { .. o!() }
        (I!(_1, [X : Xs]),)    => O!(Sx, Xs)           = { .. o!() } let { Sx = Succ(X,) } for { Sx, X, Xs }
        (I!(Z, Xs),)           => O!(_1, [Pz : Xs])    = { .. o!() } let { Pz = Pred(Z,) } for { Pz, Xs, Z }
}

// Mul2 ////////////////////////////////////////////////////////////////////////

ty! {
    fam Mul2 :: Fn(Nat) -> Nat where
        (_1,)        => O!(_1, [])       = { .. o!() }
        (O!(X, Xs),) => O!(Sx, Xs)       = { .. o!() } let { Sx = Succ(X,) } for { Sx, X, Xs }
        (I!(X, Xs),) => O!(_1, [X : Xs]) = { .. o!() } for { X, Xs }
}

// Div2 ////////////////////////////////////////////////////////////////////////

ty! {
    fam Div2 :: Fn(Nat) -> Nat where
        (O!(_1, []),)        => _1         = { ..   _1 }
        (O!(_1, [X : Xs]),)  => I!(X, Xs)  = { .. i!() } let { Sx = Succ(X,) } for { Sx, X, Xs }
        (O!(O!(Y, Ys), Xs),) => O!(Px, Xs) = { .. o!() } let { Px = Succ((Y, Ys),) } for { Px, Xs, Y, Ys }
        (O!(I!(Y, Ys), Xs),) => O!(Px, Xs) = { .. o!() } let { Px = Succ((Y, Ys),) } for { Px, Xs, Y, Ys }
}
