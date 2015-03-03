#![allow(unused_features)]
#![feature(core)]
#![feature(on_unimplemented)]
#![feature(plugin)]
#![feature(unboxed_closures)]
#![plugin(shoggoth_plugins)]

// #![feature(log_syntax)]
// #![feature(trace_macros)]

// log_syntax!();
// trace_macros!(true);

#[macro_use] mod macros;

pub mod bit;
pub mod bool;
pub mod diff;
pub mod list;
pub mod order;

pub mod numerics;
pub mod products;
pub mod reflect;
