pub use bit::*;

mod pos {
    use bit::*;
    pub trait Pos {}
    impl Pos for _1 {}
    impl<P: Pos, B> Pos for (P, B) {}
}

