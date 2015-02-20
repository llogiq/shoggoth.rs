#![crate_type="dylib"]
#![feature(collections)]
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
use syntax::parse::{
    lexer,
    parser,
    token,
};

#[plugin_registrar]
#[doc(hidden)]
pub fn shoggoth_plugins_registrar(reg: &mut plugin::Registry) {
    reg.register_macro("invoke_for_seq_upto", invoke_for_seq_upto_expand);
    reg.register_macro("Val", val_expand);
}

fn invoke_for_seq_upto_expand<'cx>(
    ecx: &'cx mut base::ExtCtxt,
    span: codemap::Span,
    args: &[ast::TokenTree],
) -> Box<base::MacResult + 'cx> {
    let mut parser = ecx.new_parser_from_tts(args);

    // parse the macro name
    let mac = parser.parse_ident();

    // parse a comma
    parser.expect(&token::Token::Comma);

    // parse the number of iterations
    if let ast::Lit_::LitInt(lit, _) = parser.parse_lit().node {
        Some(lit)
    } else {
        None
    }.and_then(|iterations| {

        // generate a token tree: A0, ..., An
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

        // iterate over the ctx and generate impl syntax fragments
        let mut items = vec![];
        let mut i = ctx.len();
        for _ in range(0, iterations) {
            items.push(quote_item!(ecx, $mac!{ $ctx };).unwrap());
            i -= 2;
            ctx.truncate(i);
        }

        // splice the impl fragments into the ast
        Some(base::MacItems::new(items.into_iter()))

    }).unwrap_or_else(|| {
        ecx.span_err(span, "invoke_for_seq_upto!: expected an integer literal argument");
        base::DummyResult::any(span)
    })
}

// convert a u64 to a string representation of a type-level binary natural, e.g.,
//     to_bin_nat(1024)
//         ==> (((((((((_1, _0), _0), _0), _0), _0), _0), _0), _0), _0)
#[inline(always)]
fn to_bin_nat(mut num: u64) -> String {
    let mut res = String::from_str("_");
    if num < 2 {
        res.push_str(num.to_string().as_slice());
    } else {
        let mut bin = vec![];
        while num > 0 {
            bin.push(num % 2);
            num >>= 1;
        }
        res = ::std::iter::repeat('(').take(bin.len() - 1).collect();
        res.push_str("_");
        res.push_str(bin.pop().unwrap().to_string().as_slice());
        for b in bin.iter().rev() {
            res.push_str(", _");
            res.push_str(b.to_string().as_slice());
            res.push_str(")");
        }
    }
    return res;
}

// generate a parser to convert a string representation of a type-level natural
// to an ast fragment for a type
#[inline(always)]
pub fn bin_nat_parser<'cx>(
    ecx: &'cx mut base::ExtCtxt,
    num: u64,
) -> parse::parser::Parser<'cx> {
    let filemap = ecx
        .codemap()
        .new_filemap(String::from_str("<val!>"), to_bin_nat(num));
    let reader  = lexer::StringReader::new(
        &ecx.parse_sess().span_diagnostic,
        filemap);
    parser::Parser::new(
        ecx.parse_sess(),
        ecx.cfg(),
        Box::new(reader))
}

// Expand Val!(n) to a type-level binary nat where n is an int literal, e.g.,
//     Val!(1024)
//         ==> (((((((((_1, _0), _0), _0), _0), _0), _0), _0), _0), _0)
#[inline]
pub fn val_expand<'cx>(
     ecx: &'cx mut base::ExtCtxt,
    span: codemap::Span,
    args: &[ast::TokenTree],
) -> Box<base::MacResult + 'cx> {
    let mut litp = ecx.new_parser_from_tts(args);
    if let ast::Lit_::LitInt(lit, _) = litp.parse_lit().node {
        Some(lit)
    } else {
        None
    }.and_then(|lit| {
        let mut natp = bin_nat_parser(ecx, lit);
        Some(base::MacTy::new(natp.parse_ty()))
    }).unwrap_or_else(|| {
        ecx.span_err(span, "Val!: expected an integer literal argument");
        base::DummyResult::any(span)
    })
}
