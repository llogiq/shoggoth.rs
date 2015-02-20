pub use bit::*;

pub mod pos {
    use bit::*;

    pub trait Pos {}
    impl Pos for _1 {}
    impl<P: Pos, B> Pos for (P, B) {}
}

pub trait Nat {}
impl Nat for _0 {}
impl Nat for _1 {}
impl<P: pos::Pos, B> Nat for (P, B) {}

#[cfg(test)]
mod test {
    use bit;

    #[test]
    fn welp() {
        let _: Nat!(42) = nat!(42);
    }
}
