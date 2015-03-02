#![crate_type="dylib"]
#![feature(collections)]
#![feature(core)]
#![feature(plugin_registrar)]
#![feature(quote)]
#![feature(rustc_private)]

extern crate syntax;
extern crate rustc;

use rustc::plugin;

mod numerics;
mod products;

#[plugin_registrar]
#[doc(hidden)]
pub fn shoggoth_plugins_registrar(reg: &mut plugin::Registry) {
    reg.register_macro("invoke_for_seq_upto", products::invoke_for_seq_upto_expand);
    reg.register_macro("Nat", numerics::nat::expand_ty);
    reg.register_macro("nat", numerics::nat::expand_tm);
}
