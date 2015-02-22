#![feature(core)]
#![feature(on_unimplemented)]
#![feature(plugin)]
#![feature(unboxed_closures)]
#![plugin(shoggoth_plugins)]

#[macro_use] mod macros;

pub mod bit;
pub mod impls;
pub mod list;
pub mod nat;
pub mod ops;
pub mod order;
