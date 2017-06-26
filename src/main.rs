#[macro_use]
extern crate nom;

use std::str;
use nom::{IResult, space, hex_digit};

named!(statement, take_until_either_and_consume!("."));

fn is_pn_chars_base(c: char) -> bool {
    let u = c as u32;
    (u >=    0x41 && u <=    0x5A) || // [A-Z]
    (u >=    0x61 && u <=    0x7A) || // [a-z]
    (u >=    0xC0 && u <=    0xD6) ||
    (u >=    0xD8 && u <=    0xF6) ||
    (u >=    0xF8 && u <=   0x2FF) ||
    (u >=   0x370 && u <=   0x37D) ||
    (u >=   0x37F && u <=  0x1FFF) ||
    (u >=  0x200C && u <=  0x200D) ||
    (u >=  0x2C00 && u <=  0x2FEF) ||
    (u >=  0x3001 && u <=  0xD7FF) ||
    (u >=  0xF900 && u <=  0xFDCF) ||
    (u >=  0xFDF0 && u <=  0xFFFD) ||
    (u >= 0x10000 && u <= 0xEFFFF)
}
fn is_pn_chars_u(c: char) -> bool {
    is_pn_chars_base(c) || c == '_'
}
fn is_pn_chars(c: char) -> bool {
    let u = c as u32;
    is_pn_chars_u(c)             ||
    u == '-' as u32              ||
    u ==  0xB7                   ||
    (u >=   0x30 && u <=   0x39) || // [0-9]
    (u >=  0x300 && u <=  0x36F) ||
    (u >= 0x203F && u <= 0x2040)
}

/* [163s] PN_CHARS_BASE */
named!(pn_chars_base<&str, &str>, verify!(
    take_s!(1),
    |val:&str| {
        is_pn_chars_base(val.chars().next().unwrap())
    }
));

/* [164s] PN_CHARS_U */
named!(pn_chars_u<&str, &str>, verify!(
    take_s!(1),
    |val:&str| {
        is_pn_chars_u(val.chars().next().unwrap())
    }
));

/* [166s] PN_CHARS */
named!(pn_chars<&str, &str>, verify!(
    take_s!(1),
    |val:&str| {
        is_pn_chars(val.chars().next().unwrap())
    }
));

/* [169s] PLX */
named!(plx, alt!(percent | pn_local_esc));

/* [170s] PERCENT */
named!(percent, do_parse!(
    tag!("%") >>
    hex2: flat_map!(
        take!(2),
        hex_digit
    ) >>
    (hex2)
));

/* [172s]  PN_LOCAL_ESC */
named!(pn_local_esc, escaped!(
    call!(space),
    '\\',
    one_of!("_~.-!$&'()*+,;=/?#@%")
));

fn main() {
    assert_eq!(statement(&b"a b c. d e f."[..]), IResult::Done(&b" d e f."[..], &b"a b c"[..]));
    assert_eq!(pn_chars_base("a%2Ab")          , IResult::Done("%2Ab"         , "a"          ));
    assert_eq!(pn_chars_u("_a")                , IResult::Done("a"            , "_"          ));
    assert_eq!(pn_chars("-_a")                 , IResult::Done("_a"           , "-"          ));
    assert_eq!(plx(&b"%2Abc"[..])              , IResult::Done(&b"bc"[..]     , &b"2A"[..]   ));
    assert_eq!(plx(&b"\\.%2A"[..])             , IResult::Done(&b"%2A"[..]    , &b"\\."[..]  ));
    assert_eq!(percent(&b"%2Abc"[..])          , IResult::Done(&b"bc"[..]     , &b"2A"[..]   ));
    assert_eq!(pn_local_esc(&b"\\.a b c"[..])  , IResult::Done(&b"a b c"[..]  , &b"\\."[..]  ));
}
