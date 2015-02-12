use hlist::*;
use hlist::{
    self,
};
use ty::{
    _0,
    Ap,
    Ap1,
    Cmp1,
    Eval,
    Eval1,
    Infer,
    MkStore,
    infer,
    nat,
};

/// Type-level zippers for lists
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Zipper {}

/// Zipper constructor
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MkZipper<Xs, Ys>(Xs, Ys);

/// Type-level list to zipper
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Zip {}

impl Infer for Zip {
    type Arity = HC<(), HN>;
    type Mode = infer::mode::Constant;
}

/// `zip(xs) ==> mk_zipper(nil, xs)`
impl<Xs> Eval<Zip> for
    HC<Xs, HN>
{
    type Out = MkZipper<HN, Xs>;
}

/// Type-level zipper to list
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Unzip {}

impl Infer for Unzip {
    type Arity = HC<(), HN>;
    type Mode = infer::mode::Constant;
}

/// `unzip(mk_zipper(xs, ys)) ==> append(reverse(xs), ys)`
impl<Xs, Ys, Rec> Eval<Unzip> for
    HC<MkZipper<Xs, Ys>,
    HN>
where
      Xs: hlist::ReversePrepend<Ys, Out = Rec>,
{
    type Out = Rec;
}

/// Type-level move right for zipper
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Right {}

impl Infer for Right {
    type Arity = HC<(), HC<(), HN>>;
    type Mode = infer::mode::Constant;
}

impl<Xs, Ys> Eval<Right> for
    HC<_0,
    HC<MkZipper<Xs, Ys>,
    HN>>
{
    type Out = MkZipper<Xs, Ys>;
}

impl<P: nat::pos::Pos, Rec0, Rec1, Xs, Y, Ys> Eval<Right> for
    HC<P,
    HC<MkZipper<Xs, Cons<Y, Ys>>,
    HN>>
where
       P: Eval1<nat::Pred, Out = Rec0>,
    HC<Rec0,
    HC<MkZipper<Cons<Y, Xs>, Ys>,
    HN>>
        : Eval<Right, Out = Rec1>,
{
    type Out = Rec1;
}

/// Type-level get from focus of zipper
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Get {}

impl Infer for Get {
    type Arity = HC<(), HN>;
    type Mode = infer::mode::Constant;
}

/// `get(mk_zipper(xs, cons(y, ys))) ==> y`
impl<Xs, Y, Ys> Eval<Get> for
    HC<MkZipper<Xs, HC<Y, Ys>>, HN>
{
    type Out = Y;
}

/// Type-level put new element, replacing focus of zipper
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Put {}

impl Infer for Put {
    type Arity = HC<(), HC<(), HN>>;
    type Mode = infer::mode::Constant;
}

/// `put(mk_zipper(xs, cons(y, ys)), e) ==> mk_zipper(xs, cons(e, ys))`
impl<E, Xs, Y, Ys> Eval<Put> for
    HC<MkZipper<Xs, HC<Y, Ys>>, HC<E, HN>>
{
    type Out = MkZipper<Xs, HC<E, Ys>>;
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ToStore {}

impl Infer for ToStore {
    type Arity = HC<(), HN>;
    type Mode = infer::mode::Constant;
}

impl<Xs, Y, Ys> Eval<ToStore> for
    HC<MkZipper<Xs, Cons<Y, Ys>>,
    HN>
{
    type Out = MkStore<
        Ap<Get, Self>,
        Cmp1<Ap<Put, Self>, Unzip>,
    >;
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Nth {}

impl Infer for Nth {
    type Arity = HC<(), HN>;
    type Mode = infer::mode::Constant;
}

impl<N> Eval<Nth> for
    HC<N,
    HN>
{
    type Out = Cmp1<Zip, Cmp1<Ap1<Right, N>, ToStore>>;
}

#[cfg(test)]
mod test {
    use hlist::{
        HC,
        HN,
    };
    use ty::*;

    #[test]
    fn put() {
        let x0 = Witness::<Ap<zipper::Put,
                              HC<MkZipper<HC<bool, HN>,
                                          HC<u8,   HN>>,
                              HC<u16, HN>>>>;
        let x1= Witness::<MkZipper<HC<bool, HN>,
                                   HC<u16,  HN>>>;
        x0 == x1;
        let x2 = Witness::<Ap1<Unzip,
                               MkZipper<HC<bool, HN>,
                                        HC<u16,  HN>>>>;
        let x3 = Witness::<HC<bool,
                           HC<u16,
                           HN>>>;
        x2 == x3;
    }

    #[test]
    fn nth_over() {
        let x0 = Witness::<
            Ap1<Ap1<Over<lens::Cmp<Ap1<zipper::Nth, _1b>,
                                   Ap1<zipper::Nth, _1b>>>,
                    Ap1<Const, FF>>,
                HC<HC<FF, HN>,
                HC<HC<FF, HC<TT, HC<FF, HN>>>,
                HN>>>>;
        let x1 = Witness::<
                HC<HC<FF, HN>,
                HC<HC<FF, HC<FF, HC<FF, HN>>>,
                HN>>>;
        x0 == x1;
    }

    #[test]
    fn nth_set() {
        let x0 = Witness::<
            Ap1<Ap1<lens::Set<lens::Cmp<Ap1<zipper::Nth, _1b>,
                                        Ap1<zipper::Nth, _1b>>>,
                    TT>,
                HC<HC<FF, HN>,
                HC<HC<FF, HC<TT, HC<FF, HN>>>,
                HN>>>>;
        let x1 = Witness::<
                HC<HC<FF, HN>,
                HC<HC<FF, HC<TT, HC<FF, HN>>>,
                HN>>>;
        x0 == x1;
    }
}
