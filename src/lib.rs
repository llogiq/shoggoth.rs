#![feature(core)]
#![feature(hash)]
#![feature(on_unimplemented)]
#![feature(plugin)]
#![feature(unboxed_closures)]
#![plugin(shoggoth_macros)]

#[macro_use] extern crate shoggoth_macros;
#[macro_use] mod macros;

pub mod bit;
pub mod impls;
pub mod list;
pub mod nat;
pub mod ops;
