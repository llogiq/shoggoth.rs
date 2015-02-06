use hlist::{
    Cons,
    HC,
    HList,
    HN,
    Nil,
    self,
};
use ty::{
    AppEval,
    Ar,
    Ar1,
    Eval,
    Infer,
    Tm,
    Ty,
    infer,
};

/// Type-level lists
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum List<A: Ty> {}

/// ```ignore
/// A :: Ty
/// -------------
/// List<A> :: Ty
/// ```
impl<A: Ty> Ty for List<A> {}

/// ```ignore
/// A :: Ty
/// -------------
/// nil : List<A>
/// ```
impl<A: Ty> Tm<List<A>> for Nil {}

/// ```ignore
/// A :: Ty
/// x : A
/// xs : List<A>
/// ---------------------
/// cons(x, xs) : List<A>
/// ```
impl<
       A: Ty,
       X: Tm<A>,
      Xs: Tm<List<A>> + HList,
> Tm<List<A>> for Cons<X, Xs> {}

/// Type-level append for lists
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Prepend<A: Ty> {}

/// ```ignore
/// A :: Ty
/// l : List<A>
/// r : List<A>
/// ----------------------
/// append(l, r) : List<A>
/// ```
impl<A: Ty> Infer for Prepend<A> {
    type Mode = infer::mode::Constant;
    type Ty = Ar<HC<List<A>, HC<List<A>, HN>>, List<A>>;
}

impl<
       A: Ty,
      Xs: Tm<List<A>>,
      Ys: Tm<List<A>>,
     Rec: Tm<List<A>>,
> Eval<Prepend<A>> for HC<Xs, HC<Ys, HN>> where
      Xs: hlist::Prepend<Ys, Out = Rec>,
{
    type Out = Rec;
}



/// Type-level reverse for lists
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Reverse<A: Ty> {}

/// ```ignore
/// A :: Ty
/// xs : List<A>
/// ---------------------
/// reverse(xs) : List<A>
/// ```
impl< A: Ty> Infer for Reverse<A> {
    type Mode = infer::mode::Constant;
    type Ty = Ar1<List<A>, List<A>>;
}

impl<
       A: Ty,
     Rec: Tm<List<A>> + HList,
      Xs: Tm<List<A>> + HList,
> Eval<Reverse<A>> for HC<Xs, HN> where
      Xs: hlist::Reverse<Out = Rec>,
{
    type Out = Rec;
}

/// Type-level operation mapping operations over lists
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Map<A: Ty, B: Ty> {}

impl<A: Ty, B: Ty> Infer for Map<A, B> {
    type Mode = infer::mode::Constant;
    type Ty = Ar<HC<Ar<HC<A, HN>, B>, HC<List<A>, HN>>, List<B>>;
}

// `map(fx, nil) ==> nil`
impl<
       A: Ty,
       B: Ty,
      Fx: Infer<Ty = Ar1<A, B>>,
> Eval<Map<A, B>> for HC<Fx, HC<Nil, HN>> {
    type Out = Nil;
}

// `map(fx, cons(x, xs)) ==> cons(fx(x), map(fx, xs))`
impl<
       A: Ty,
       B: Ty,
      Fx: Infer<Mode = FxM, Ty = Ar1<A, B>>,
     FxM,
       X: Tm<A>,
      Xs: Tm<List<A>> + HList,
    Rec0: Tm<B>,
    Rec1: Tm<List<B>> + HList,
> Eval<Map<A, B>> for HC<Fx, HC<Cons<X, Xs>, HN>> where
    // fx(h) ==> r0
   HC<X, HN>
        : AppEval<FxM, HC<A, HN>, Fx, Out = Rec0>,
    // map(fx, t) ==> r1
    HC<Fx, HC<Xs, HN>>
        : Eval<Map<A, B>, Out = Rec1>,
{
    type Out = Cons<Rec0, Rec1>;
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NonEmpty<A: Ty> {}
impl<A: Ty> Ty for NonEmpty<A> {}
impl<A: Ty, X: Tm<A>, Xs: Tm<List<A>>> Tm<NonEmpty<A>> for HC<X, Xs> {}
