#[macro_use]
extern crate nom;

use nom::{IResult, space};

named!(statement, take_until_either_and_consume!("."));
named!(percent<(char, char)>, do_parse!(
    tag!("%") >>
    a: hex    >>
    b: hex    >>
    (a, b)
));
named!(hex<char>, one_of!("0123456789ABCDEFabcdef"));
named!(pn_local_esc, escaped!(
    call!(space),
    '\\',
    one_of!("_~.-!$&\'()*+,;=/?#@%")
));

fn main() {
    assert_eq!(statement(&b"a b c. d e f."[..]), IResult::Done(&b" d e f."[..], &b"a b c"[..]));
    assert_eq!(percent(&b"%2Abc"[..]),           IResult::Done(&b"bc"[..]     , ('2', 'A')   ));
    assert_eq!(hex(&b"Abc"[..])                , IResult::Done(&b"bc"[..]     , 'A'          ));
    assert_eq!(pn_local_esc(&b"\\.a b c"[..])  , IResult::Done(&b"a b c"[..]  , &b"\\."[..]  ));
}
