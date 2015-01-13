use ty::{
    Tm,
    Ty,
    bool,
    fun,
};

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Nat {}
impl Ty for Nat {}

/// Type-level natural number zero
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Zero {}
impl Tm<Nat> for Zero {}

/// Type-level natural number successor of n
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Succ<N: Tm<Nat>> {}
impl<N: Tm<Nat>> Tm<Nat> for Succ<N> {}

/// Type-level function for nat predecessor
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Pred {}
impl fun::Sig for Pred { type Dom = (Nat,); type Cod = Nat; }
impl<N: Tm<Nat>> fun::Fn<Pred> for (Succ<N>,)
{
    type O = N;
}

/// Type-level function for nat addition
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Add {}
impl fun::Sig for Add { type Dom = (Nat, Nat,); type Cod = Nat; }
impl<N1: Tm<Nat>> fun::Fn<Add> for (Zero, N1,)
{
    type O = N1;
}
impl<N0: Tm<Nat>, N1: Tm<Nat>> fun::Fn<Add> for (Succ<N0>, N1,) where
    (N0, N1,): fun::Fn<Add>,
    fun::Ap<Add, (N0, N1,)>: Tm<Nat>,
{
    type O = Succ<fun::Ap<Add, (N0, N1,)>>;
}

/// Type-level function for nat subtraction
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Sub {}
impl fun::Sig for Sub { type Dom = (Nat, Nat,); type Cod = Nat; }
impl<N0: Tm<Nat>> fun::Fn<Sub> for (N0, Zero,)
{
    type O = N0;
}
impl<N0: Tm<Nat>, N1: Tm<Nat>, Rec: Tm<Nat>> fun::Fn<Sub> for (Succ<N0>, Succ<N1>,) where
    (N0, N1,): fun::Fn<Sub, O = Rec>,
{
    type O = Rec;
}

/// Type-level function for nat multiplication
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Mul {}
impl fun::Sig for Mul { type Dom = (Nat, Nat,); type Cod = Nat; }
impl<N1: Tm<Nat>> fun::Fn<Mul> for (Zero, N1,)
{
    type O = Zero;
}
impl<N0: Tm<Nat>, N1: Tm<Nat>, Rec: Tm<Nat>> fun::Fn<Mul> for (Succ<N0>, N1,) where
    (N0, N1,): fun::Fn<Mul, O = Rec>,
    (N1, Rec,): fun::Fn<Add>,
    fun::Ap<Add, (N1, Rec,)>: Tm<Nat>,
{
    type O = fun::Ap<Add, (N1, Rec,)>;
}

/// Type-level function for nat exponentiation
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Exp {}
impl fun::Sig for Exp { type Dom = (Nat, Nat,); type Cod = Nat; }
impl<N1: Tm<Nat>> fun::Fn<Exp> for (Zero, N1,)
{
    type O = Succ<Zero>;
}
impl<N0: Tm<Nat>, N1: Tm<Nat>, Rec: Tm<Nat>> fun::Fn<Exp> for (Succ<N0>, N1,) where
    (N0, N1,): fun::Fn<Exp, O = Rec>,
    (N1, Rec,): fun::Fn<Mul>,
    fun::Ap<Mul, (N1, Rec,)>: Tm<Nat>,
{
    type O = fun::Ap<Mul, (N1, Rec,)>;
}

/// Type-level function for nat factorial
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Fac {}
impl fun::Sig for Fac { type Dom = (Nat,); type Cod = Nat; }
impl fun::Fn<Fac> for (Zero,)
{
    type O = Succ<Zero>;
}
impl<N0: Tm<Nat>, Rec: Tm<Nat>> fun::Fn<Fac> for (Succ<N0>,) where
    (N0,): fun::Fn<Fac, O = Rec>,
    (Succ<N0>, Rec,): fun::Fn<Mul>,
    fun::Ap<Mul, (Succ<N0>, Rec,)>: Tm<Nat>,
{
    type O = fun::Ap<Mul, (Succ<N0>, Rec,)>;
}

/// Type-level function for nat less-than
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum LT {}
impl fun::Sig for LT { type Dom = (Nat, Nat,); type Cod = bool::Bool; }
impl<N1: Tm<Nat>> fun::Fn<LT> for (Zero, N1,) {
    type O = bool::True;
}
impl<N0: Tm<Nat>> fun::Fn<LT> for (Succ<N0>, Zero,) {
    type O = bool::False;
}
impl<N0: Tm<Nat>, N1: Tm<Nat>> fun::Fn<LT> for (Succ<N0>, Succ<N1>,) where
    (N0, N1,): fun::Fn<LT>,
    fun::Ap<LT, (N0, N1,)>: Tm<bool::Bool>,
{
    type O = fun::Ap<LT, (N0, N1,)>;
}

/// Type-level function for nat less-than-or-eq
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum LTEq {}
impl fun::Sig for LTEq { type Dom = (Nat, Nat,); type Cod = bool::Bool; }
impl fun::Fn<LTEq> for (Zero, Zero,) {
    type O = bool::True;
}
impl<N0: Tm<Nat>> fun::Fn<LTEq> for (Succ<N0>, Zero,) {
    type O = bool::False;
}
impl<N1: Tm<Nat>> fun::Fn<LTEq> for (Zero, Succ<N1>,) {
    type O = bool::True;
}
impl<N0: Tm<Nat>, N1: Tm<Nat>> fun::Fn<LTEq> for (Succ<N0>, Succ<N1>,) where
    (N0, N1,): fun::Fn<LTEq>,
    fun::Ap<LTEq, (N0, N1,)>: Tm<bool::Bool>,
{
    type O = fun::Ap<LTEq, (N0, N1,)>;
}

// /// Type-level function for nat min
// #[derive(Clone)]
// #[derive(Copy)]
// #[derive(Eq)]
// #[derive(Hash)]
// #[derive(Ord)]
// #[derive(PartialEq)]
// #[derive(PartialOrd)]
// #[derive(Show)]
// pub enum Min {}
// impl fun::Sig for Min { type Dom = (Nat, Nat,); type Cod = Nat; }
// impl<N0: Tm<Nat>, N1: Tm<Nat>, Rec: Tm<bool::Bool>> fun::Fn<Min> for (N0, N1,) where
//     (N0, N1): fun::Fn<LTEq, O = Rec>,
//     (Rec, N0, N1,): fun::Fn<bool::If<Nat>>,
//     fun::Ap<bool::If<Nat>, (Rec, N0, N1,)>: Tm<Nat>,
// {
//     type O = fun::Ap<bool::If<Nat>, (Rec, N0, N1,)>;
// }

#[cfg(test)]
mod tests {
    use ty::literal::*;
    use ty::nat::*;
    use ty::val::*;

    #[test]
    fn pred() { let _: Val<_1n> = val::<Pred, (_2n,)>(); }

    #[test]
    fn add() { let _: Val<_5n> = val::<Add, (_3n, _2n,)>(); }

    #[test]
    fn sub() { let _: Val<_3n> = val::<Sub, (_5n, _2n,)>(); }

    #[test]
    fn mul() { let _: Val<_6n> = val::<Mul, (_3n, _2n,)>(); }

    #[test]
    fn exp() { let _: Val<_8n> = val::<Exp, (_3n, _2n,)>(); }

    #[test]
    fn fac() { let _: Val<_24n> = val::<Fac, (_4n,) >(); }

    #[test]
    fn lt_false() { let _: Val<FF> = val::<LT, (_4n, _2n,)>(); }

    #[test]
    fn lt_true() { let _: Val<TT> = val::<LT, (_2n, _4n,)>(); }

    #[test]
    fn lteq_false() { let _: Val<FF> = val::<LTEq, (_4n, _2n,)>(); }

    #[test]
    fn lteq_true_00() { let _: Val<TT> = val::<LTEq, (_2n, _2n,)>(); }

    #[test]
    fn lteq_true_01() { let _: Val<TT> = val::<LTEq, (_2n, _4n,)>(); }

    // #[test]
    // fn min_lhs() { let _: Val<_2n> = val::<Min, (_2n, _4n,)>(); }

    // #[test]
    // fn min_rhs() { let _: Val<_2n> = val::<Min, (_4n, _2n,)>(); }
}
