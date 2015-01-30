use hlist::*;
use ty::{
    Tm,
    Ty,
};

/// ```ignore
/// ---------
/// Nil :: Ty
/// ```
impl
    Ty
for
    Nil
{}

/// ```ignore
/// HTy :: Ty
/// TTy :: HList
/// TTy :: Ty
/// --------------------
/// Cons<HTy, TTy> :: Ty
/// ```
impl<
    HTy,
    TTy,
>
    Ty
for
    Cons<HTy, TTy>
where
    HTy: Ty,
    TTy: HList + Ty,
{}



/// ```ignore
/// ---------
/// Nil : Nil
/// ```
impl
    Tm<Nil>
for
    Nil
{}

/// ```ignore
/// HTm : HTy
/// HTy :: Ty
/// TTm :: HList, TTm : TTy
/// TTy :: HList, TTy :: Ty
/// -------------------------------
/// Cons<HTm, TTm> : Cons<HTy, TTy>
/// ```
impl<
    HTm,
    HTy,
    TTm,
    TTy,
>
    Tm<Cons<HTy, TTy>>
for
    Cons<HTm, TTm>
where
    HTm: Tm<HTy>,
    HTy: Ty,
    TTm: HList,
    TTm: Tm<TTy>,
    TTy: HList,
    TTy: Ty,
{}



pub trait
    Prefix<T>
where
    Self: HList,
    T: HList,
    T: Ty,
{
    type Out: Ty;
}

impl<
    T,
>
    Prefix<T>
for
    HN
where
    T: HList,
    T: Ty,
{
    type Out = T;
}

impl<
    HTm,
    HTy,
    TTm,
    TTy,
>
    Prefix<HC<HTy, TTy>>
for
    HC<HTm, TTm>
where
    HTm: Tm<HTy>,
    HTy: Ty,
    TTm: Prefix<TTy>,
    TTm: HList,
    TTy: HList,
    TTy: Ty,
{
    type Out = <TTm as Prefix<TTy>>::Out;
}
