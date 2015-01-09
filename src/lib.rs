#![allow(staged_experimental)]


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

pub use self::singleton::{
    Singleton,
};

pub use self::squash::{
    Squash,
};

pub use self::tuple::{
    IsComposite,
    TupleOps,
};

mod hlist;
mod product;
mod singleton;
mod squash;
mod tuple;

/// "Higher-kinded Types"
pub mod hkt;

/// Type-level programming
pub mod ty;
