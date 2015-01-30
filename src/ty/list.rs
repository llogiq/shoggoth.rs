use hlist::*;
use ty::{
    Arrow,
    Eval,
    Infer,
    Tm,
    Ty,
    infer,
};

/// Type-level lists
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
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
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
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
    type Ty = Arrow<HC<List<A>, HC<List<A>, HN>>, List<A>>;
}

/// `append(nil, r) => r`
impl<
    A,
    R,
>
    Eval<Append<A>>
for
    HC<HN, HC<R, HN>>
where
    A: Ty,
    R: Tm<List<A>>,
{
    type Out = R;
}

/// `append(cons(lh, lt), r) => cons(lh, append(lt, r))`
impl<
    A,
    LH,
    LT,
    R,
    Rec,
>
    Eval<Append<A>>
for
    HC<HC<LH, LT>, HC<R, HN>>
where
    A: Ty,
    LH: Tm<A>,
    LT: Tm<List<A>> + HList,
    R: Tm<List<A>>,
    Rec: Tm<List<A>>,
    HC<LT, HC<R, HN>>: Eval<Append<A>, Out = Rec>,
{
    type Out = HC<LH, Rec>;
}



/// Type-level operation mapping operations over lists
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
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
    type Ty = Arrow<HC<Arrow<HC<A, HN>, B>, HC<List<A>, HN>>, List<B>>;
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
    Fx: Infer<Ty = Arrow<HC<A, HN>, B>>,
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
    Fx: Infer<Ty = Arrow<HC<A, HN>, B>>,
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
