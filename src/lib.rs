#![feature(core)]
#![feature(hash)]
#![feature(on_unimplemented)]
#![feature(plugin)]
#![feature(unboxed_closures)]
#![plugin(shoggoth_macros)]

#[macro_use] extern crate shoggoth_macros;

pub mod bit;
#[macro_use] pub mod list;
pub mod nat;

pub mod op;
pub mod syntax;
