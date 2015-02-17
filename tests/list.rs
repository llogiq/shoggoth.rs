#[macro_use] extern crate shoggoth;

#[cfg(test)]
mod product {
    use super::shoggoth::syntax::product::{
        ProductOps,
    };

    #[test]
    fn to_list() {
        let xs = list![0u8, true, "foo", Some(42u64), Ok::<_, ()>(false)];
        assert_eq!(xs.to_list(), xs);
    }

    #[test]
    fn to_tuple() {
        assert_eq!(list![0u8, true, "foo", Some(42u64), Ok::<_, ()>(false)].to_tuple(),
                        (0u8, true, "foo", Some(42u64), Ok::<_, ()>(false)));
    }
}
