#![feature(core)]
#![feature(hash)]
#![feature(rand)]
#![feature(on_unimplemented)]

pub use self::hlist::{
    Cons,
    HC,
    HList,
    HN,
    IsCons,
    Nil,
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
/// Heterogeneous lists

/// Type-level programming
pub mod ty;
