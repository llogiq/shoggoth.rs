#![feature(associated_types)]

///
// pub trait DepFn0 {
//     type Out;
//     fn apply() -> Self;
// }

// pub trait DepFn1 {
//     type Out;
//     fn apply(self) -> Self::Out;
// }

// pub trait DepFn2<U> {
//     type Out;
//     fn apply(self, u: U) -> Self::Out;
// }
///

pub mod equality;
pub mod hlist;
pub mod products;
pub mod tuples;

#[cfg(test)]
mod tests {
    use products::{
        ProductOps,
    };
    use tuples::{
        TupleOps,
    };

    #[test]
    fn tuple_head() {
        let x = (0u8,);
        x.head();

        let x = (0u8, 0u8,);
        x.head();

        let x = (0u8, 0u8, 0u8,);
        x.head();

        let x = (0u8, 0u8, 0u8, 0u8,);
        x.head();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8,);
        x.head();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.head();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.head();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.head();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.head();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.head();
    }

    #[test]
    fn tuple_tail() {
        let x = (0u8,);
        x.tail();

        let x = (0u8, 0u8,);
        x.tail();

        let x = (0u8, 0u8, 0u8,);
        x.tail();

        let x = (0u8, 0u8, 0u8, 0u8,);
        x.tail();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8,);
        x.tail();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.tail();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.tail();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.tail();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.tail();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.tail();
    }

    #[test]
    fn tuple_to_hlist() {
        let x = (0u8,);
        x.to_hlist();

        let x = (0u8, 0u8,);
        x.to_hlist();

        let x = (0u8, 0u8, 0u8,);
        x.to_hlist();

        let x = (0u8, 0u8, 0u8, 0u8,);
        x.to_hlist();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8,);
        x.to_hlist();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.to_hlist();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.to_hlist();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.to_hlist();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.to_hlist();

        let x = (0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,);
        x.to_hlist();
    }
}
