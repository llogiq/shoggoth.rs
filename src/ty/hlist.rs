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
     HTy: Ty,
     TTy: HList + Ty,
>
    Ty
for
    Cons<HTy, TTy>
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
     HTm: Tm<HTy>,
     HTy: Ty,
     TTm: Tm<TTy> + HList,
     TTy: Ty      + HList,
>
    Tm<Cons<HTy, TTy>>
for
    Cons<HTm, TTm>
{}



/// ```ignore
/// n <= m
/// forall k. k <= n -> M_k : Tm<A_k>
/// --------------------------------------------
/// [ M_0, ..., M_n ] : TmPre<[ A_0, ..., A_m ]>
/// ```
pub trait
    TmPre<A>
where
    Self:      HList,
       A: Ty + HList,
{
    type Out: Ty;
}

/// ```ignore
/// A :: Ty
/// A :: HList (i.e., is a product type)
/// --------------
/// Nil : TmPre<A>
/// ```
impl<
       A: Ty + HList,
>
    TmPre<A>
for
    HN
{
    type Out = A;
}

/// ```ignore
/// HTy :: Ty
/// TTy :: Ty      + HList (i.e., is a product type)
/// HTm :: Tm<HTy>
/// TTm :: Tm<TTy> + HList (i.e., is a product term)
/// TmPre<TTy> holds for TTm
/// --------------------------------
/// Cons<HTm, TTm> : TmPre<HTy, TTy>
/// ```
impl<
     HTm: Tm<HTy>,
     HTy: Ty,
     TTm:      HList,
     TTy: Ty + HList,
>
    TmPre<HC<HTy, TTy>>
for
    HC<HTm, TTm>
where
     TTm: TmPre<TTy>,
{
    type Out = <TTm as TmPre<TTy>>::Out;
}
}
