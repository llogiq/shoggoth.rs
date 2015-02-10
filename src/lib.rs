#![feature(hash)]
#![feature(on_unimplemented)]
#![feature(plugin)]

#[plugin] extern crate shoggoth_macros;

macro_rules! seq_head {
    ($x:ident) => { $x };
    ($x:ident, $($xs:ident),*) => { $x };
}

macro_rules! seq_tail {
    ($x:ident) => { () };
    ($x:ident, $($xs:ident),*) => { ($($xs,)*) };
}

// pub use self::hlist::{
//     Cons,
//     HC,
//     HList,
//     HN,
//     IsCons,
//     Nil,
// };
// pub use self::product::{
//     ProductOps,
//     ToHList,
//     ToTuple,
// };
// pub use self::tuple::{
//     IsComposite,
//     TupleOps,
// };

/// Heterogeneous lists
pub mod hlist;
// mod product;
// mod tuple;

/// Type-level programming
pub mod ty;
