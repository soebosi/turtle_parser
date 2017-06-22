#[macro_use]
extern crate nom;

use nom::{IResult, space, hex_digit};

named!(statement, take_until_either_and_consume!("."));

fn is_pn_chars_base(chr: &u8) -> bool {
    (*chr >=    0x41 && *chr <=    0x5A) ||
    (*chr >=    0x61 && *chr <=    0x7A) ||
    (*chr >=    0xC0 && *chr <=    0xD6) ||
    (*chr >=    0xD8 && *chr <=    0xF6) ||
    (*chr >=    0xF8 && *chr <=   0x2FF) ||
    (*chr >=   0x370 && *chr <=   0x37D) ||
    (*chr >=   0x37F && *chr <=  0x1FFF) ||
    (*chr >=  0x200C && *chr <=  0x200D) ||
    (*chr >=  0x2C00 && *chr <=  0x2FEF) ||
    (*chr >=  0x3001 && *chr <=  0xD7FF) ||
    (*chr >=  0xF900 && *chr <=  0xFDCF) ||
    (*chr >=  0xFDF0 && *chr <=  0xFFFD) ||
    (*chr >= 0x10000 && *chr <= 0xEFFFF)
}
fn _pn_chars_base(input:&[u8]) -> IResult<&[u8],&[u8]> {
    for c in input {
        if ! is_pn_chars_base(c) {
            return IResult::Error(nom::ErrorKind::Custom(20))
        }
    }
    IResult::Done(&input[1..], &input[0..1])
}
/* [163s] PN_CHARS_BASE */
named!(pn_chars_base, flat_map!(
    take!(1),
    _pn_chars_base
));

/* [169s] PLX */
named!(plx, alt!(percent | pn_local_esc));

/* [170s] PERCENT */
named!(percent, do_parse!(
    tag!("%") >>
    a: flat_map!(
        take!(2),
        hex_digit
    ) >>
    (a)
));

/* [172s]  PN_LOCAL_ESC */
named!(pn_local_esc, escaped!(
    call!(space),
    '\\',
    one_of!("_~.-!$&'()*+,;=/?#@%")
));

fn main() {
    assert_eq!(statement(&b"a b c. d e f."[..]), IResult::Done(&b" d e f."[..], &b"a b c"[..]));
    assert_eq!(pn_chars_base(&b"a%2Ab"[..]),     IResult::Done(&b"%2Ab"[..]   , &b"a"[..]    ));
    assert_eq!(plx(&b"%2Abc"[..]),               IResult::Done(&b"bc"[..]     , &b"2A"[..]   ));
    assert_eq!(plx(&b"\\.%2A"[..]),              IResult::Done(&b"%2A"[..]    , &b"\\."[..]  ));
    assert_eq!(percent(&b"%2Abc"[..]),           IResult::Done(&b"bc"[..]     , &b"2A"[..]   ));
    assert_eq!(pn_local_esc(&b"\\.a b c"[..])  , IResult::Done(&b"a b c"[..]  , &b"\\."[..]  ));
}
