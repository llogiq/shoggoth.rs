use hlist::{
    Cons,
    HC,
    HList,
    HN,
    Nil,
    self,
};
use ty::{
    Ar,
    Ar1,
    Eval,
    Infer,
    Tm,
    Ty,
    infer,
};

/// Type-level lists
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    List<A>
where
    A: Ty,
{}

/// ```ignore
/// A :: Ty
/// -------------
/// List<A> :: Ty
/// ```
impl<
    A,
>
    Ty
for
    List<A>
where
    A: Ty,
{}

/// ```ignore
/// A :: Ty
/// -------------
/// nil : List<A>
/// ```
impl<
    A,
>
    Tm<List<A>>
for
    HN
where
    A: Ty,
{}

/// ```ignore
/// A :: Ty
/// h : A
/// t : List<A>
/// --------------------
/// cons(h, t) : List<A>
/// ```
impl<
    A,
    H,
    T,
>
    Tm<List<A>>
for
    HC<H, T>
where
    A: Ty,
    H: Tm<A>,
    T: Tm<List<A>>,
{}



/// Type-level append for lists
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Append<A>
where
    A: Ty,
{}

/// ```ignore
/// A :: Ty
/// l : List<A>
/// r : List<A>
/// ----------------------
/// append(l, r) : List<A>
/// ```
impl<
    A,
>
    Infer
for
    Append<A>
where
    A: Ty,
{
    type Mode = infer::mode::Constant;
    type Ty = Ar<HC<List<A>, HC<List<A>, HN>>, List<A>>;
}

impl<
    A,
    L,
    R,
    Rec,
>
    Eval<Append<A>>
for
    HC<L, HC<R, HN>>
where
    A: Ty,
    L: hlist::Append<R, Out = Rec>,
    L: Tm<List<A>>,
    R: Tm<List<A>>,
    Rec: Tm<List<A>>,
{
    type Out = Rec;
}



/// Type-level reverse for lists
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Reverse<A>
where
    A: Ty,
{}

/// ```ignore
/// A :: Ty
/// xs : List<A>
/// ---------------------
/// reverse(xs) : List<A>
/// ```
impl<
    A,
>
    Infer
for
    Reverse<A>
where
    A: Ty,
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<List<A>, List<A>>;
}

impl<
    A,
    Rec,
    Xs,
>
    Eval<Reverse<A>>
for
    HC<Xs, HN>
where
    A: Ty,
    Rec: Tm<List<A>>,
    Xs: HList,
    Xs: hlist::Reverse<Out = Rec>,
    Xs: Tm<List<A>>,
{
    type Out = Rec;
}



/// Type-level operation mapping operations over lists
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Map<A, B>
where
    A: Ty,
    B: Ty,
{}

impl<
    A,
    B,
>
    Infer
for
    Map<A, B>
where
    A: Ty,
    B: Ty,
{
    type Mode = infer::mode::Constant;
    type Ty = Ar<HC<Ar<HC<A, HN>, B>, HC<List<A>, HN>>, List<B>>;
}

// `map(fx, nil) => nil`
impl<
    A,
    B,
    Fx,
>
    Eval<Map<A, B>>
for
    HC<Fx, HC<Nil, HN>>
where
    A: Ty,
    B: Ty,
    Fx: Infer<Ty = Ar1<A, B>>,
{
    type Out = Nil;
}

// `map(fx, cons(h, t)) => cons(fx(h), map(f, t))`
impl<
    A,
    B,
    Fx,
    H,
    T,
    Rec0,
    Rec1,
>
    Eval<Map<A, B>>
for
    HC<Fx, HC<Cons<H, T>, HN>>
where
    A: Ty,
    B: Ty,
    Fx: Infer<Ty = Ar1<A, B>>,
    H: Tm<A>,
    T: HList,
    T: Tm<List<A>>,

    // fx(h) => r0
    HC<H, HN>: Eval<Fx, Out = Rec0>,
    // map(fx, t) => r1
    HC<Fx, HC<T, HN>>: Eval<Map<A, B>, Out = Rec1>,
    // cons(r0, r1) => out
    Cons<Rec0, Rec1>: Tm<List<B>>,
{
    type Out = Cons<Rec0, Rec1>;
}
