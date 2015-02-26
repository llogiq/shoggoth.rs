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

pub mod numerics;

pub mod impls;
pub mod list;
pub mod ops;
pub mod order;
pub mod reflect;
