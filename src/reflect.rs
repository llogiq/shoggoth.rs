pub trait Reifies {
    type Output;
    fn reflect(&self) -> Self::Output;
}

pub trait Reflects {
    fn reify(&self) -> Box<Reifies<Output = Self>>;
}
