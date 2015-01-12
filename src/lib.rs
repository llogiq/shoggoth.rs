#![allow(unstable)]

extern crate unify;

pub use self::hlist::{
    HC,
    HList,
    HN,
    IsHCons,
};

pub use self::product::{
    ProductOps,
    ToHList,
    ToTuple,
};

pub use self::tuple::{
    IsComposite,
    TupleOps,
};

mod hlist;
mod product;
mod tuple;

/// Type-level programming
pub mod ty;
