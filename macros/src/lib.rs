#![crate_type="dylib"]
#![feature(core)]
#![feature(plugin_registrar)]
#![feature(quote)]
#![feature(rustc_private)]

extern crate syntax;
extern crate rustc;

use rustc::plugin;
use syntax::{
    ast,
    codemap,
    parse,
};
use syntax::ext::base;
use syntax::ext::quote::rt::{
    ToTokens,
};
use syntax::parse::token;

#[macro_export] macro_rules! HList {
    {} => { ::hlist::Nil };
    { $head:ty } => { ::hlist::Cons<$head, ::hlist::Nil> };
    { $head:ty, $($tail:ty),* } => { ::hlist::Cons<$head, HList!($($tail),*)> };
}

#[macro_export] macro_rules! hlist {
    {} => { ::hlist::Nil };
    { $head:expr } => { ::hlist::Cons($head, ::hlist::Nil) };
    { $head:expr, $($tail:expr),* } => { ::hlist::Cons($head, hlist!($($tail),*)) };
}

#[macro_export] macro_rules! seq_head {
    { $x:ident } => { $x };
    { $x:ident, $($xs:ident),* } => { $x };
}

#[macro_export] macro_rules! seq_tail {
    { $x:ident } => { () };
    { $x:ident, $($xs:ident),* } => { ($($xs,)*) };
}

#[plugin_registrar]
pub fn impl_for_seq_upto_registrar(reg: &mut plugin::Registry) -> ()
{
    reg.register_macro("impl_for_seq_upto", impl_for_seq_upto_expand);
}

fn impl_for_seq_upto_expand<'cx>(
    ecx: &'cx mut base::ExtCtxt,
    span: codemap::Span,
    args: &[ast::TokenTree],
) -> Box<base::MacResult + 'cx> {
    let mut parser = ecx.new_parser_from_tts(args);

    // parse the macro name
    let mac = parser.parse_ident();

    // parse a comma
    parser.eat(&token::Token::Comma);

    // parse the number of iterations
    let iterations = match parser.parse_lit().node {
        ast::Lit_::LitInt(i, _) => i,
        _ => {
            ecx.span_err(span, "welp");
            return base::DummyResult::any(span);
        }
    };

    let mut ctx = range(0, iterations * 2 - 1).flat_map(|k| {
        if k % 2 == 0 {
            token::str_to_ident(format!("A{}", (k / 2)).as_slice())
                .to_tokens(ecx)
                .into_iter()
        } else {
            let span  = codemap::DUMMY_SP;
            let token = parse::token::Token::Comma;
            vec![ast::TokenTree::TtToken(span, token)]
                .into_iter()
        }
    }).collect::<Vec<_>>();

    let mut items = vec![];
    let mut i = ctx.len();
    for _ in range(0, iterations) {
        items.push(quote_item!(ecx, $mac!{ $ctx };).unwrap());
        i -= 2;
        ctx.truncate(i);
    }

    base::MacItems::new(items.into_iter())
}
