#![crate_type="dylib"]
#![feature(core)]
#![feature(plugin_registrar)]
#![feature(quote)]
#![feature(rustc_private)]

extern crate syntax;
extern crate rustc;

use rustc::plugin;

mod seq;

#[plugin_registrar]
#[doc(hidden)]
pub fn shoggoth_plugins_registrar(reg: &mut plugin::Registry) {
    reg.register_macro("invoke_for_seq_upto", seq::expand_invoke_for_seq_upto);
}
