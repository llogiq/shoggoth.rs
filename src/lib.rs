#![allow(unstable)]

extern crate hkt;
extern crate unify;

// pub use self::functor::{
//     Functor,
// };

pub use self::hlist::{
    HCons,
    HList,
    HNil,
    IsHCons,
};

// pub use self::monad::{
//     Monad,
// };

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

// mod functor;
mod hlist;
// mod monad;
mod product;
mod squash;
mod tuple;

/// Type-level programming
pub mod ty;
