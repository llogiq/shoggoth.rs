#![feature(hash)]
#![feature(on_unimplemented)]
#![feature(plugin)]
#![recursion_limit="66"]

#[macro_use] #[plugin] extern crate shoggoth_macros;

pub use self::product::{
    ProductOps,
    ToHList,
    ToTuple,
};
pub use self::tuple::{
    IsComposite,
    TupleOps,
};

/// Heterogeneous lists
pub mod hlist;
mod product;
mod tuple;

/// Type-level programming
pub mod ty;
