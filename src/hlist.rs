#[deriving(Copy)]
pub struct HNil;
pub struct HCons<X, Xs: HList> {
    pub car: X,
    pub cdr: Xs,
}

pub trait HList {
    #[inline]
    fn cons<X>(self, x: X) -> HCons<X, Self> {
        HCons {
            car: x,
            cdr: self,
        }
    }
}

impl HList for HNil {}
impl<X, Xs: HList> HList for HCons<X, Xs> where Xs: HList {}

pub trait Head {
    type Car;
    fn head(self) -> <Self as Head>::Car;
}
impl<X, Xs: HList> Head for HCons<X, Xs> {
    type Car = X;
    #[inline]
    fn head(self) -> X {
        self.car
    }
}

pub trait Tail {
    type Cdr;
    fn tail(self) -> <Self as Tail>::Cdr;
}
impl<X, Xs: HList> Tail for HCons<X, Xs> {
    type Cdr = Xs;
    #[inline]
    fn tail(self) -> Xs {
        self.cdr
    }
}

#[cfg(test)]
mod tests {

    use super::{
        HNil,
        HCons,
        HList,
        Head,
        Tail,
    };

    #[test]
    fn nil() {
        let _: HNil = HNil;
    }
    
    #[test]
    fn cons() {
        let _: HCons<f64, HCons<Vec<String>, HCons<&str, HCons<u8, HNil>>>> = HNil.cons(0u8).cons("foo").cons(Vec::<String>::new()).cons(2.5f64);
    }

    #[test]
    fn head() {
        let _: uint = HNil.cons(42u).head();
    }

    #[test]
    fn tail() {
        let _: HCons<f64, HNil> = HNil.cons(2.0f64).cons(42u).tail();
    }
}
