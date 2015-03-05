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
