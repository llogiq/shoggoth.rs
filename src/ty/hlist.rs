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



/// ```ignore
/// n <= m
/// forall k. k <= n -> M_k : Tm<A_k>
/// -----------------------------------------------
/// [ M_0, ..., M_n ] : TmPrefix<[ A_0, ..., A_m ]>
/// ```
pub trait
    TmPrefix<A>
where
    Self: HList,
    A: HList,
    A: Ty,
{
    type Out: Ty;
}

/// ```ignore
/// A :: Ty
/// A :: HList (i.e., is a product type)
/// -----------------
/// Nil : TmPrefix<A>
/// ```
impl<
    A,
>
    TmPrefix<A>
for
    HN
where
    A: HList,
    A: Ty,
{
    type Out = A;
}

/// ```ignore
/// HTy :: Ty
/// TTy :: Ty      + HList (i.e., is a product type)
/// HTm :: Tm<HTy>
/// TTm :: Tm<TTy> + HList (i.e., is a product term)
/// TmPrefix<TTy> holds for TTm
/// -----------------------------------
/// Cons<HTm, TTm> : TmPrefix<HTy, TTy>
/// ```
impl<
    HTm,
    HTy,
    TTm,
    TTy,
>
    TmPrefix<HC<HTy, TTy>>
for
    HC<HTm, TTm>
where
    HTm: Tm<HTy>,
    HTy: Ty,
    TTm: TmPrefix<TTy>,
    TTm: HList,
    TTy: HList,
    TTy: Ty,
{
    type Out = <TTm as TmPrefix<TTy>>::Out;
}
