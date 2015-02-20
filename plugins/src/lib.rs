#![crate_type="dylib"]
#![feature(collections)]
#![feature(core)]
#![feature(plugin_registrar)]
#![feature(quote)]
#![feature(rustc_private)]

extern crate syntax;
extern crate rustc;

use rustc::plugin;

mod boilerplate;
mod numerics;

#[plugin_registrar]
#[doc(hidden)]
pub fn shoggoth_plugins_registrar(reg: &mut plugin::Registry) {
    reg.register_macro("invoke_for_seq_upto", boilerplate::invoke_for_seq_upto_expand);
    reg.register_macro("Nat", numerics::nat_ty_expand);
    reg.register_macro("nat", numerics::nat_tm_expand);
}
