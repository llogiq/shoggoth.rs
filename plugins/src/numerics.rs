use syntax::{
    ast,
    codemap,
    parse,
};
use syntax::ext::base;
use syntax::parse::{
    lexer,
    parser,
};

const NAT_STR_PRE: &'static str =   "bit::_";
const NAT_STR_SEP: &'static str = ", bit::_";

// Convert a u64 to a string representation of a type-level binary natural, e.g.,
//     nat_str(1024)
//         ==> (((((((((_1, _0), _0), _0), _0), _0), _0), _0), _0), _0)
#[inline(always)]
fn nat_str(mut num: u64) -> String {
    let mut res = String::from_str(NAT_STR_PRE);
    if num < 2 {
        res.push_str(num.to_string().as_slice());
    } else {
        let mut bin = vec![];
        while num > 0 {
            bin.push(num % 2);
            num >>= 1;
        }
        res = ::std::iter::repeat('(').take(bin.len() - 1).collect();
        res.push_str(NAT_STR_PRE);
        res.push_str(bin.pop().unwrap().to_string().as_slice());
        for b in bin.iter().rev() {
            res.push_str(NAT_STR_SEP);
            res.push_str(b.to_string().as_slice());
            res.push_str(")");
        }
    }
    return res;
}

// Generate a parser with the nat string for `num` as input
#[inline(always)]
fn nat_str_parser<'cx>(
    ecx: &'cx mut base::ExtCtxt,
    num: u64,
) -> parse::parser::Parser<'cx> {
    let filemap = ecx
        .codemap()
        .new_filemap(String::from_str("<nat!>"), nat_str(num));
    let reader  = lexer::StringReader::new(
        &ecx.parse_sess().span_diagnostic,
        filemap);
    parser::Parser::new(
        ecx.parse_sess(),
        ecx.cfg(),
        Box::new(reader))
}

// Try to parse an integer literal and return a new parser for its nat string
#[inline]
pub fn nat_lit_parser<'cx>(
     ecx: &'cx mut base::ExtCtxt,
    args: &[ast::TokenTree],
) -> Option<parse::parser::Parser<'cx>> {
    let mut litp = ecx.new_parser_from_tts(args);
    if let ast::Lit_::LitInt(lit, _) = litp.parse_lit().node {
        Some(nat_str_parser(ecx, lit))
    } else {
        None
    }
}

// Expand Nat!(n) to a type-level binary nat where n is an int literal, e.g.,
//     Nat!(1024)
//         ==> (((((((((_1, _0), _0), _0), _0), _0), _0), _0), _0), _0)
#[inline]
pub fn nat_ty_expand<'cx>(
     ecx: &'cx mut base::ExtCtxt,
    span: codemap::Span,
    args: &[ast::TokenTree],
) -> Box<base::MacResult + 'cx> {
    {
        nat_lit_parser(ecx, args)
    }.and_then(|mut natp| {
        Some(base::MacTy::new(natp.parse_ty()))
    }).unwrap_or_else(|| {
        ecx.span_err(span, "Nat!: expected an integer literal argument");
        base::DummyResult::any(span)
    })
}

// Expand nat!(n) to a term-level binary nat where n is an int literal, e.g.,
//     nat!(1024)
//         ==> (((((((((_1, _0), _0), _0), _0), _0), _0), _0), _0), _0)
#[inline]
pub fn nat_tm_expand<'cx>(
     ecx: &'cx mut base::ExtCtxt,
    span: codemap::Span,
    args: &[ast::TokenTree],
) -> Box<base::MacResult + 'cx> {
    {
        nat_lit_parser(ecx, args)
    }.and_then(|mut natp| {
        Some(base::MacExpr::new(natp.parse_expr()))
    }).unwrap_or_else(|| {
        ecx.span_err(span, "nat!: expected an integer literal argument");
        base::DummyResult::any(span)
    })
}
