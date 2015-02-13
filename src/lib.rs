#![feature(core)]
#![feature(hash)]
#![feature(on_unimplemented)]
#![feature(plugin)]
#![feature(unboxed_closures)]

#[macro_use] #[plugin] extern crate shoggoth_macros;

pub mod bit;
pub mod hlist;
pub mod nat;

pub mod op;
pub mod syntax;
