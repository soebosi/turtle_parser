#[macro_use]
extern crate nom;

use nom::{IResult};

named!(statement, take_until_either_and_consume!("."));

fn main() {
    let input = &b"a b c. d e f."[..];
    assert_eq!(statement(input), IResult::Done(&b" d e f."[..], &b"a b c"[..]));
}
