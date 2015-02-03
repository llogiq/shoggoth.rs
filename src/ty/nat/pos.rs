use hlist::*;
use ty::{
    Ar,
    Ar1,
    Eval,
    Infer,
    Tm,
    Ty,
    infer,
};
use ty::bit::*;

/// Type-level positive natural numbers (binary)
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Pos
{}

/// ```ignore
/// ---------
/// Pos :: Ty
/// ```
impl
    Ty
for
    Pos
{}

/// ```ignore
/// -------
/// 1 : Pos
/// ```
impl
    Tm<Pos>
for
    _1
{}

/// ```ignore
/// p : Pos
/// b : Bit
/// ------------
/// (p, b) : Pos
/// ```
impl<
       B: Tm<Bit>,
       P: Tm<Pos>,
>
    Tm<Pos>
for
    (P, B)
{}



/// Type-level successor for positive natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Succ
{}

/// ```ignore
/// p : Pos
/// -------------
/// succ(p) : Pos
/// ```
impl
    Infer
for
    Succ
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<Pos, Pos>;
}

/// `succ(1) ==> 1:0`
impl
    Eval<Succ>
for
    HC<_1, HN>
{
    type Out = (_1, _0,);
}

/// `succ(p:0) ==> p:1`
impl<
       P: Tm<Pos>,
>
    Eval<Succ>
for
    HC<(P, _0), HN>
{
    type Out = (P, _1);
}

/// `p:1 ==> succ(p):0`
impl<
       P: Tm<Pos>,
     Rec: Tm<Pos>,
>
    Eval<Succ>
for
    HC<(P, _1), HN>
where
       HC<P, HN>: Eval<Succ, Out = Rec>,
{
    type Out = (Rec, _0);
}



/// Type-level addition for positive natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Add
{}

/// ```ignore
/// p : Pos
/// q : Pos
/// ---------------
/// add(p, q) : Pos
/// ```
impl
    Infer
for
    Add
{
    type Mode = infer::mode::Constant;
    type Ty = Ar<HC<Pos, HC<Pos, HN>>, Pos>;
}

/// `add(1, 1) ==> 1:0`
impl
    Eval<Add>
for
    HC<_1, HC<_1, HN>>
{
    type Out = (_1, _0);
}

/// `add(1, q:0) ==> q:1`
impl<
      P1: Tm<Pos>,
>
    Eval<Add>
for
    HC<_1, HC<(P1, _0), HN>>
{
    type Out = (P1, _1);
}

/// `add(1, q:1) ==> succ(q):0`
impl<
      P1: Tm<Pos>,
     Rec: Tm<Pos>,
>
    Eval<Add>
for
    HC<_1, HC<(P1, _1), HN>>
where
      HC<P1, HN>: Eval<Succ, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `add(p:0, 1) ==> p:1`
impl<
      P0: Tm<Pos>,
>
    Eval<Add>
for
    HC<(P0, _0), HC<_1, HN>>
{
    type Out = (P0, _1);
}

/// `add(p:0, q:0) ==> add(p, q):0`
impl<
      P0: Tm<Pos>,
      P1: Tm<Pos>,
     Rec: Tm<Pos>,
>
    Eval<Add>
for
    HC<(P0, _0), HC<(P1, _0), HN>>
where
    HC<P0, HC<P1, HN>>
        : Eval<Add, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `add(p:0, q:1) ==> add(p, q):1`
impl<
      P0: Tm<Pos>,
      P1: Tm<Pos>,
     Rec: Tm<Pos>,
>
    Eval<Add>
for
    HC<(P0, _0), HC<(P1, _1), HN>>
where
    HC<P0, HC<P1, HN>>
        : Eval<Add, Out = Rec>,
{
    type Out = (Rec, _1);
}

/// `add(p:1, 1) ==> succ(p):0`
impl<
      P0: Tm<Pos>,
     Rec: Tm<Pos>,
>
    Eval<Add>
for
    HC<(P0, _1), HC<_1, HN>>
where
      HC<P0, HN>: Eval<Succ, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `add(p:1, q:0) ==> add(p, q):1`
impl<
      P0: Tm<Pos>,
      P1: Tm<Pos>,
     Rec: Tm<Pos>,
>
    Eval<Add>
for
    HC<(P0, _1), HC<(P1, _0), HN>>
where
    HC<P0, HC<P1, HN>>
        : Eval<Add, Out = Rec>,
{
    type Out = (Rec, _1);
}

/// `add(p:1, q:1) ==> add_carry(p, q):1`
impl<
      P0: Tm<Pos>,
      P1: Tm<Pos>,
     Rec: Tm<Pos>,
>
    Eval<Add>
for
    HC<(P0, _1), HC<(P1, _1), HN>>
where
    HC<P0, HC<P1, HN>>
        : Eval<AddCarry, Out = Rec>,
{
    type Out = (Rec, _0);
}



/// Type-level addition with carry for positive natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    AddCarry
{}

/// ```ignore
/// p : Pos
/// q : Pos
/// ---------------------
/// add_carry(p, q) : Pos
/// ```
impl
    Infer
for
    AddCarry
{
    type Mode = infer::mode::Constant;
    type Ty = Ar<HC<Pos, HC<Pos, HN>>, Pos>;
}

/// `add_carry(1, 1) ==> 1:1`
impl
    Eval<AddCarry>
for
    HC<_1, HC<_1, HN>>
{
    type Out = (_1, _1);
}

/// `add_carry(1, q:0) ==> succ(q):0`
impl<
      P1: Tm<Pos>,
     Rec: Tm<Pos>,
>
    Eval<AddCarry>
for
    HC<_1, HC<(P1, _0), HN>>
where
      HC<P1, HN>: Eval<Succ, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `add_carry(1, q:1) ==> succ(q):1`
impl<
      P1: Tm<Pos>,
     Rec: Tm<Pos>,
>
    Eval<AddCarry>
for
    HC<_1, HC<(P1, _1), HN>>
where
      HC<P1, HN>: Eval<Succ, Out = Rec>,
{
    type Out = (Rec, _1);
}

/// `add_carry(p:0, 1) ==> p:1`
impl<
      P0: Tm<Pos>,
     Rec: Tm<Pos>,
>
    Eval<AddCarry>
for
    HC<(P0, _0), HC<_1, HN>>
where
      HC<P0, HN>: Eval<Succ, Out = Rec>,
{
    type Out = (P0, _0);
}

/// `add_carry(p:0, q:0) ==> add(p, q):1`
impl<
      P0: Tm<Pos>,
      P1: Tm<Pos>,
     Rec: Tm<Pos>,
>
    Eval<AddCarry>
for
    HC<(P0, _0), HC<(P1, _0), HN>>
where
    HC<P0, HC<P1, HN>>
        : Eval<Add, Out = Rec>,
{
    type Out = (Rec, _1);
}

/// `add_carry(p:0, q:1) ==> add_carry(p, q):0`
impl<
      P0: Tm<Pos>,
      P1: Tm<Pos>,
     Rec: Tm<Pos>,
>
    Eval<AddCarry>
for
    HC<(P0, _0), HC<(P1, _1), HN>>
where
    HC<P0, HC<P1, HN>>
        : Eval<AddCarry, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `add_carry(p:1, 1) ==> succ(p):1`
impl<
      P0: Tm<Pos>,
     Rec: Tm<Pos>,
>
    Eval<AddCarry>
for
    HC<(P0, _1), HC<_1, HN>>
where
      HC<P0, HN>: Eval<Succ, Out = Rec>,
{
    type Out = (Rec, _1);
}

/// `add_carry(p:1, q:0) ==> add_carry(p, q):0`
impl<
      P0: Tm<Pos>,
      P1: Tm<Pos>,
     Rec: Tm<Pos>,
>
    Eval<AddCarry>
for
    HC<(P0, _1), HC<(P1, _0), HN>>
where
    HC<P0, HC<P1, HN>>
        : Eval<AddCarry, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `add_carry(p:1, q:1) ==> add_carry(p, q):1`
impl<
      P0: Tm<Pos>,
      P1: Tm<Pos>,
     Rec: Tm<Pos>,
>
    Eval<AddCarry>
for
    HC<(P0, _1), HC<(P1, _1), HN>>
where
    HC<P0, HC<P1, HN>>
        : Eval<AddCarry, Out = Rec>,
{
    type Out = (Rec, _1);
}



#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    PredDouble
{}

impl
    Infer
for
    PredDouble
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<Pos, Pos>;
}

impl
    Eval<PredDouble>
for
    HC<_1, HN>
{
    type Out = _1;
}

impl<
       P: Tm<Pos>,
     Rec: Tm<Pos>,
>
    Eval<PredDouble>
for
    HC<(P, _0), HN>
where
       HC<P, HN>: Eval<PredDouble, Out = Rec>,
{
    type Out = (Rec, _1);
}

impl<
       P: Tm<Pos>,
>
    Eval<PredDouble>
for
    HC<(P, _1), HN>
{
    type Out = ((P, _0), _1);
}



/// Type-level predecessor for positive natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Pred
{}

/// ```ignore
/// p : Pos
/// -------------
/// pred(p) : Pos
/// ```
impl
    Infer
for
    Pred
{
    type Mode = infer::mode::Constant;
    type Ty = Ar1<Pos, Pos>;
}

/// `pred(p:1) ==> p:0`
impl
    Eval<Pred>
for
    HC<_1, HN>
{
    type Out = _1;
}

/// `p:0 ==> pred_double(p)`
impl<
       P: Tm<Pos>,
     Rec: Tm<Pos>,
>
    Eval<Pred>
for
    HC<(P, _0), HN>
where
       HC<P, HN>: Eval<PredDouble, Out = Rec>,
{
    type Out = Rec;
}

/// `p:1 ==> p:0`
impl<
       P: Tm<Pos>,
>
    Eval<Pred>
for
    HC<(P, _1), HN>
{
    type Out = (P, _0);
}



/// Type-level multiplication for positive natural numbers
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum
    Mul
{}

/// ```ignore
/// p : Pos
/// q : Pos
/// ---------------
/// mul(p, q) : Pos
/// ```
impl
    Infer
for
    Mul
{
    type Mode = infer::mode::Constant;
    type Ty = Ar<HC<Pos, HC<Pos, HN>>, Pos>;
}

/// `mul(1, q) ==> q`
impl<
      P1: Tm<Pos>,
>
    Eval<Mul>
for
    HC<_1, HC<P1, HN>>
{
    type Out = P1;
}

/// `mul(p:0, q) ==> mul(p, q):0`
impl<
      P0: Tm<Pos>,
      P1: Tm<Pos>,
     Rec: Tm<Pos>,
>
    Eval<Mul>
for
    HC<(P0, _0), HC<P1, HN>>
where
    HC<P0, HC<P1, HN>>
        : Eval<Mul, Out = Rec>,
{
    type Out = (Rec, _0);
}

/// `mul(p:1, q) ==> add(q, mul(p, q)):0`
impl<
      P0: Tm<Pos>,
      P1: Tm<Pos>,
    Rec0: Tm<Pos>,
    Rec1: Tm<Pos>,
>
    Eval<Mul>
for
    HC<(P0, _1), HC<P1, HN>>
where
    // mul(p0, p1) ==> r0
    HC<P0, HC<P1, HN>>
        : Eval<Mul, Out = Rec0>,
    // mul(p1, r0) ==> r1
    HC<P1, HC<Rec0, HN>>
        : Eval<Add, Out = Rec1>,
{
    type Out = Rec1;
}



#[cfg(test)]
mod test {
    use super::*;
    use ty::*;

    #[test]
    fn pred() {
        let x0: Witness<Ap1<nat::pos::Pred, (_1, _0)>> = Witness;
        let x1: Witness<_1> = Witness;
        x0 == x1;
    }
}
