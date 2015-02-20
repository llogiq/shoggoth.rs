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
fn bin_nat_parser<'cx>(
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
