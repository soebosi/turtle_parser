#[macro_use]
extern crate nom;

use nom::{IResult, space};

named!(statement, take_until_either_and_consume!("."));

named!(percent, do_parse!(
    char!('%')        >>
    a: take!(2)       >>
    (a)
));
named!(pn_local_esc, escaped!(
    call!(space),
    '\\',
    one_of!("_~.-!$&\'()*+,;=/?#@%")
));

fn main() {
    assert_eq!(statement(&b"a b c. d e f."[..]), IResult::Done(&b" d e f."[..], &b"a b c"[..]));
    assert_eq!(percent(&b"%2Abc"[..])          , IResult::Done(&b"bc"[..]     , &b"2A"[..])   );
    assert_eq!(pn_local_esc(&b"\\.a b c"[..])  , IResult::Done(&b"a b c"[..]  , &b"\\."[..])  );
}
