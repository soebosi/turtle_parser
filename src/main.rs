#[macro_use]
extern crate nom;

use std::str;
use nom::{IResult, space, hex_digit};

named!(statement, take_until_either_and_consume!("."));

fn is_pn_chars_base(c: char) -> bool {
    let u = c as u32;
    match u {
        0x41    ... 0x5A   | 0x61   ... 0x7A   |
        0xC0    ... 0xD6   | 0xD8   ... 0xF6   |
        0xF8    ... 0x2FF  | 0x370  ... 0x37D  |
        0x37F   ... 0x1FFF | 0x200C ... 0x200D |
        0x2C00  ... 0x2FEF | 0x3001 ... 0xD7FF |
        0xF900  ... 0xFDCF | 0xFDF0 ... 0xFFFD |
        0x10000 ... 0xEFFFF
          => true,
        _ => false,
    }
}
fn is_pn_chars_u(c: char) -> bool {
    is_pn_chars_base(c) || c == '_'
}
fn is_pn_chars(c: char) -> bool {
    let u = c as u32;
    match u {
        0x2d | 0xB7 | 0x30...0x39 | 0x300...0x36F | 0x203F...0x2040
          => true,
        _ => is_pn_chars_base(c),
    }
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
