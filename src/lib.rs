#![feature(core)]
#![feature(hash)]
#![feature(rand)]
#![feature(on_unimplemented)]

pub use self::hlist::{
    HCons,
    HList,
    HNil,
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
