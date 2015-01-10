#![allow(unstable)]

pub use self::functor::{
    Functor,
};

pub use self::hlist::{
    HCons,
    HList,
    HNil,
    IsHCons,
};

pub use self::monad::{
    Monad,
};

pub use self::product::{
    ProductOps,
    ToHList,
    ToTuple,
};

pub use self::squash::{
    Squash,
};

pub use self::tuple::{
    IsComposite,
    TupleOps,
};

mod functor;
mod hlist;
mod monad;
mod product;
mod squash;
mod tuple;

/// "Higher-kinded Types"
pub mod hkt;

/// Type-level programming
pub mod ty;
