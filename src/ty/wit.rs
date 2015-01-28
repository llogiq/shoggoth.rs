use ty::{
    Act,
    Eval,
    Rule,
    Sig,
    Tm,
};

/// A structure for witnessing a type-level computation
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Rand)]
#[derive(Show)]
pub struct
    Wit<A>;

impl<
    X,
>
    Wit<X>
{
    /// Compute a type-level expression by applying a type-level partial
    /// operation `Op` to input term given as `X`
    #[inline]
    pub fn app<Op>(self) -> Wit<Eval<Act<Op>, X>> where
        Op: Sig,
        X: Rule<Op>,
        X: Tm<<Op as Sig>::Dom>,
    {
        Wit
    }
}
