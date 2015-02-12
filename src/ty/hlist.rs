use hlist::*;

pub trait TmPre<A> {
    type Out;
}

impl<A> TmPre<A> for HN {
    type Out = A;
}

impl<
     HTm,
     HTy,
     TTm,
     TTy,
> TmPre<HC<HTy, TTy>> for HC<HTm, TTm> where
     TTm: TmPre<TTy>
{
    type Out = <TTm as TmPre<TTy>>::Out;
}

pub trait TmExt<A> {
    type Out; // FIXME: + Prepend<Self::Ext, Out = Self>;
    type Ext;
}

impl<M> TmExt<HN> for M {
    type Out = HN;
    type Ext = M;
}

impl<
     HTm,
     HTy,
  RecExt,
  RecOut,
     TTm,
     TTy,
> TmExt<HC<HTy, TTy>> for HC<HTm, TTm> where
     TTm: TmExt<TTy, Out = RecOut, Ext = RecExt>,
{
    type Out = HC<HTm, RecOut>;
    type Ext = RecExt;
}

#[cfg(test)]
mod test {
    use hlist::*;
    use ty::*;

    #[test]
    fn tm_ext() {
        let x0 = Witness::<<HC<TT, HC<FF, HN>> as TmExt<HC<(), HN>>>::Out>;
        let x1 = Witness::<HC<TT, HN>>;
        x0 == x1;
        let x2 = Witness::<<HC<TT, HC<FF, HN>> as TmExt<HC<(), HN>>>::Ext>;
        let x3 = Witness::<HC<FF, HN>>;
        x2 == x3;
    }
}
