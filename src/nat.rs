use function::{
    TFn,
};

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum TZ {}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum TS<TN: TNat> {}

pub trait TNat {}
impl TNat for TZ {}
impl<TN: TNat> TNat for TS<TN> {}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Add {}
impl<TN: TNat> TFn<Add, (TZ, TN)> for Add
{
    type Out = TN;
}
impl<TN0: TNat, TN1: TNat> TFn<Add, (TS<TN0>, TN1)> for Add where
    Add: TFn<Add, (TN0, TN1)>,
{
    type Out = TS<<Add as TFn<Add, (TN0, TN1)>>::Out>;
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Mul {}
impl<TN1: TNat> TFn<Mul, (TZ, TN1)> for Mul
{
    type Out = TZ;
}
impl<TN0: TNat, TN1: TNat, Rec: TNat> TFn<Mul, (TS<TN0>, TN1)> for Mul where
    Mul: TFn<Mul, (TN0, TN1), Out = Rec>,
    Add: TFn<Add, (TN1, Rec)>,
{
    type Out = <Add as TFn<Add, (TN1, Rec)>>::Out;
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Exp {}
impl<TN1: TNat> TFn<Exp, (TZ, TN1)> for Exp
{
    type Out = TS<TZ>;
}
impl<TN0: TNat, TN1: TNat, Rec: TNat> TFn<Exp, (TS<TN0>, TN1)> for Exp where
    Exp: TFn<Exp, (TN0, TN1), Out = Rec>,
    Mul: TFn<Mul, (TN1, Rec)>,
{
    type Out = <Mul as TFn<Mul, (TN1, Rec)>>::Out;
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Ord)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Show)]
pub enum Fac {}
impl TFn<Fac, TZ> for Fac
{
    type Out = TS<TZ>;
}
impl<TN: TNat, Rec: TNat> TFn<Fac, TS<TN>> for Fac where
    Fac: TFn<Fac, TN, Out = Rec>,
    Mul: TFn<Mul, (TS<TN>, Rec)>,
{
    type Out = <Mul as TFn<Mul, (TS<TN>, Rec)>>::Out;
}

pub type TN00 = TZ;
pub type TN01 = TS<TN00>;
pub type TN02 = TS<TN01>;
pub type TN03 = TS<TN02>;
pub type TN04 = TS<TN03>;
pub type TN05 = TS<TN04>;
pub type TN06 = TS<TN05>;
pub type TN07 = TS<TN06>;
pub type TN08 = TS<TN07>;
pub type TN09 = TS<TN08>;
pub type TN10 = TS<TN09>;
pub type TN11 = TS<TN10>;
pub type TN12 = TS<TN11>;
pub type TN13 = TS<TN12>;
pub type TN14 = TS<TN13>;
pub type TN15 = TS<TN14>;
pub type TN16 = TS<TN15>;
pub type TN17 = TS<TN16>;
pub type TN18 = TS<TN17>;
pub type TN19 = TS<TN18>;
pub type TN20 = TS<TN19>;
pub type TN21 = TS<TN20>;
pub type TN22 = TS<TN21>;
pub type TN23 = TS<TN22>;
pub type TN24 = TS<TN23>;
pub type TN25 = TS<TN24>;
pub type TN26 = TS<TN25>;
pub type TN27 = TS<TN26>;
pub type TN28 = TS<TN27>;
pub type TN29 = TS<TN28>;
pub type TN30 = TS<TN29>;
pub type TN31 = TS<TN30>;
pub type TN32 = TS<TN31>;
pub type TN33 = TS<TN32>;
pub type TN34 = TS<TN33>;
pub type TN35 = TS<TN34>;
pub type TN36 = TS<TN35>;
pub type TN37 = TS<TN36>;
pub type TN38 = TS<TN37>;
pub type TN39 = TS<TN38>;
pub type TN40 = TS<TN39>;
pub type TN41 = TS<TN40>;
pub type TN42 = TS<TN41>;
pub type TN43 = TS<TN42>;
pub type TN44 = TS<TN43>;
pub type TN45 = TS<TN44>;
pub type TN46 = TS<TN45>;
pub type TN47 = TS<TN46>;
pub type TN48 = TS<TN47>;
pub type TN49 = TS<TN48>;
pub type TN50 = TS<TN49>;
pub type TN51 = TS<TN50>;
pub type TN52 = TS<TN51>;
pub type TN53 = TS<TN52>;
pub type TN54 = TS<TN53>;
pub type TN55 = TS<TN54>;
pub type TN56 = TS<TN55>;
pub type TN57 = TS<TN56>;
pub type TN58 = TS<TN57>;
pub type TN59 = TS<TN58>;
pub type TN60 = TS<TN59>;
pub type TN61 = TS<TN60>;
pub type TN62 = TS<TN61>;
pub type TN63 = TS<TN62>;
pub type TN64 = TS<TN63>;
pub type TN65 = TS<TN64>;
pub type TN66 = TS<TN65>;
pub type TN67 = TS<TN66>;
pub type TN68 = TS<TN67>;
pub type TN69 = TS<TN68>;
pub type TN70 = TS<TN69>;
pub type TN71 = TS<TN70>;
pub type TN72 = TS<TN71>;
pub type TN73 = TS<TN72>;
pub type TN74 = TS<TN73>;
pub type TN75 = TS<TN74>;
pub type TN76 = TS<TN75>;
pub type TN77 = TS<TN76>;
pub type TN78 = TS<TN77>;
pub type TN79 = TS<TN78>;
pub type TN80 = TS<TN79>;
pub type TN81 = TS<TN80>;
pub type TN82 = TS<TN81>;
pub type TN83 = TS<TN82>;
pub type TN84 = TS<TN83>;
pub type TN85 = TS<TN84>;
pub type TN86 = TS<TN85>;
pub type TN87 = TS<TN86>;
pub type TN88 = TS<TN87>;
pub type TN89 = TS<TN88>;
pub type TN90 = TS<TN89>;
pub type TN91 = TS<TN90>;
pub type TN92 = TS<TN91>;
pub type TN93 = TS<TN92>;
pub type TN94 = TS<TN93>;
pub type TN95 = TS<TN94>;
pub type TN96 = TS<TN95>;
pub type TN97 = TS<TN96>;
pub type TN98 = TS<TN97>;
pub type TN99 = TS<TN98>;

#[cfg(test)]
mod tests {
    use function::{
        TVal,
    };
    use super::{
        Add,
        Mul,
        Exp,
        Fac,
        TN02,
        TN03,
        TN04,
        TN05,
        TN06,
        TN08,
        TN24,
    };

    #[test]
    fn add() {
        let _: TVal<TN05> = TVal::val::<Add, (TN03, TN02)>();
    }

    #[test]
    fn mul() {
        let _: TVal<TN06> = TVal::val::<Mul, (TN03, TN02)>();
    }

    #[test]
    fn exp() {
        let _: TVal<TN08> = TVal::val::<Exp, (TN03, TN02)>();
    }

    #[test]
    fn fac() {
        let _: TVal<TN24> = TVal::val::<Fac, TN04>();
    }
}
