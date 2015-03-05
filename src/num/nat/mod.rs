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
