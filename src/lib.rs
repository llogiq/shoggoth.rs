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

#[cfg(feature = "numerics")]
pub mod numerics;

#[cfg(feature = "products")]
pub mod products;

#[cfg(feature = "reflection")]
pub mod reflect;

pub mod bit;
pub mod bool;
pub mod order;
