#![feature(box_syntax)]
#![feature(core)]
#![feature(plugin)]
#![feature(unboxed_closures)]
#![plugin(shoggoth_plugins)]

// #![feature(log_syntax)]
// #![feature(trace_macros)]

// log_syntax!();
// trace_macros!(true);

#[macro_use] mod macros;

pub mod data;
pub mod num;
