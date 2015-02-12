use hlist::{
    Cons,
    HC,
    HN,
    Nil,
    self,
};
use ty::{
    AppEval,
    Eval,
    Infer,
    infer,
};

/// Type-level lists
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum List {}

/// Type-level append for lists
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Prepend {}

impl Infer for Prepend {
    type Arity = HC<(), HC<(), HN>>;
    type Mode = infer::mode::Constant;
}

impl<Xs, Ys, Rec> Eval<Prepend> for HC<Xs, HC<Ys, HN>> where
      Xs: hlist::Prepend<Ys, Out = Rec>,
{
    type Out = Rec;
}

/// Type-level reverse for lists
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Reverse {}

impl Infer for Reverse {
    type Arity = HC<(), HN> ;
    type Mode = infer::mode::Constant;
}

impl<Rec, Xs> Eval<Reverse> for HC<Xs, HN> where
      Xs: hlist::Reverse<Out = Rec>,
{
    type Out = Rec;
}

/// Type-level operation mapping operations over lists
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Map {}

impl Infer for Map {
    type Arity = HC<HC<(), HN>, HC<(), HN>>;
    type Mode = infer::mode::Constant;
}

// `map(fx, nil) ==> nil`
impl<Fx> Eval<Map> for HC<Fx, HC<Nil, HN>>
{
    type Out = Nil;
}

// `map(fx, cons(x, xs)) ==> cons(fx(x), map(fx, xs))`
impl<Fx, FxM, X, Xs, Rec0, Rec1> Eval<Map> for HC<Fx, HC<Cons<X, Xs>, HN>> where
      Fx: Infer<Mode = FxM>,
    // fx(h) ==> r0
    HC<X, HN>
        : AppEval<FxM, HC<(), HN>, Fx, Out = Rec0>,
    // map(fx, t) ==> r1
    HC<Fx, HC<Xs, HN>>
        : Eval<Map, Out = Rec1>,
{
    type Out = Cons<Rec0, Rec1>;
}

#[cfg(test)]
mod test {
    use hlist::*;
    use ty::*;

    #[test]
    fn map() {
        let x0 = Witness::<Ap<list::Map, HC<Not, HC<HC<FF, HC<TT, HN>>, HN>>>>;
        let x1 = Witness::<HC<TT, HC<FF, HN>>>;
        x0 == x1;
    }
}
