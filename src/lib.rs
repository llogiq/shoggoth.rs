#![feature(associated_types)]

pub mod equality;
pub mod hlist;
pub mod product;
pub mod singleton;
pub mod squash;
pub mod tuple;

#[cfg(test)]
mod tests {
    use equality::{
        Is,
    };
    use product::{
        ProductOps,
    };
    use tuple::{
        TupleOps,
    };

    #[test]
    fn equality_coerce() {
        fn aux<X, Y: Is<X>>(y: Y) -> X {
            y.coerce()
        }
        assert_eq!((), aux::<(), ()>(()))
    }

    // FIXME: Need compile-time #[should_fail]
    /*
    #[test]
    #[should_fail]
    fn equality_coerce_fail() {
        fn aux<X, Y: Is<X>>(y: Y) -> X {
            y.coerce()
        }
        aux::<(), bool>(())
    }
     */

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
