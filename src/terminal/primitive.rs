extern crate nom;

use std::str;
use nom::{
    is_hex_digit,
    digit,
    alpha,
    alphanumeric,
};

fn is_ws(c: char) -> bool {
    let u = c as u32;
    u == 0x20 || u == 0x9 || u == 0xD || u == 0xA
}

fn is_anon(c: char) -> bool {
    c == '[' || c == ']' || is_ws(c)
}

#[derive(PartialEq, Debug)]
pub struct Langtag<'a> {
    language: &'a str,
    subtags:  Vec<&'a str>,
}
/* [144s] LANGTAG */
named!(pub langtag<&str, Langtag>, do_parse!(
    tag!("@") >>
    language: alpha >>
    tag!("-") >>
    subtags:  separated_list_complete!(
        tag!("-"),
        alphanumeric
    ) >>
    (Langtag{
        language: language,
        subtags:  subtags,
    })
));

#[derive(PartialEq, Debug)]
pub struct Integer<'a> {
    sign:          Option<&'a str>,
    integer_part:  &'a str,
}
/* [19] INTEGER */
named!(pub integer<&str, Integer>, do_parse!(
    sign:         opt!(alt!(tag!("+") | tag!("-"))) >>
    integer_part: digit                             >>
    (Integer{
        sign:          sign,
        integer_part:  integer_part,
    })
));

#[derive(PartialEq, Debug)]
pub struct Decimal<'a> {
    sign:          Option<&'a str>,
    integer_part:  Option<&'a str>,
    decimal_point: &'a str,
    decimal_part:  &'a str,
}
/* [20] DECIMAL */
named!(pub decimal<&str, Decimal>, do_parse!(
    sign:          opt!(alt!(tag!("+") | tag!("-"))) >>
    integer_part:  opt!(digit)                       >>
    decimal_point: tag!(".")                         >>
    decimal_part:  digit                             >>
    (Decimal{
        sign:          sign,
        integer_part:  integer_part,
        decimal_point: decimal_point,
        decimal_part:  decimal_part,
    })
));

#[derive(PartialEq, Debug)]
pub enum Mantissa<'a> {
    IntegerDecimal {
        integer_part:  &'a str,
        decimal_point: &'a str,
        decimal_part:  Option<&'a str>,
    },
    Decimal {
        decimal_point: &'a str,
        decimal_part:  &'a str,
    },
    Integer {
        integer_part: &'a str,
    },
}
#[derive(PartialEq, Debug)]
pub struct Double<'a> {
    sign:     Option<&'a str>,
    mantissa: Mantissa<'a>,
    exponent: Exponent<'a>,
}
/* [21] DOUBLE */
named!(pub double<&str, Double>, do_parse!(
    sign:     opt!(alt!(tag!("+") | tag!("-"))) >>
    mantissa: alt!(
        do_parse!(
            integer_part:  digit       >>
            decimal_point: tag!(".")   >>
            decimal_part:  opt!(digit) >>
            (Mantissa::IntegerDecimal{
                integer_part:  integer_part,
                decimal_point: decimal_point,
                decimal_part:  decimal_part,
            })
        ) |
        do_parse!(
            decimal_point: tag!(".") >>
            decimal_part:  digit     >>
            (Mantissa::Decimal{
                decimal_point: decimal_point,
                decimal_part:  decimal_part,
            })
        ) |
        do_parse!(
            integer_part: digit >>
            (Mantissa::Integer{
                integer_part: integer_part,
            })
        )
    ) >>
    exponent: exponent >>
    (Double{
        sign:     sign,
        mantissa: mantissa,
        exponent: exponent,
    })
));

#[derive(PartialEq, Debug)]
pub struct Exponent<'a> {
    exponent_symbol: &'a str,
    sign:            Option<&'a str>,
    exponent_number: &'a str,
}

/* [154s] EXPONENT */
named!(pub exponent<&str, Exponent>, do_parse!(
    exponent_symbol: alt!(tag!("e") | tag!("E"))       >>
    sign:            opt!(alt!(tag!("+") | tag!("-"))) >>
    exponent_number: digit                             >>
    (Exponent{
        exponent_symbol: exponent_symbol,
        sign:            sign,
        exponent_number: exponent_number,
    })
));

/* [161s] WS */
named!(pub ws<&str, &str>, take_while_s!(is_ws));

/* [162s] ANON */
named!(pub anon<&str, &str>, verify!(
    take_while_s!(is_anon),
    |val:&str| {
        let len = val.char_indices().count();
        val.char_indices().all(|(idx, c)| {
            if idx == 0 {
                c == '['
            } else if idx == len - 1 {
                c == ']'
            } else {
                is_ws(c)
            }
        })
    }
));

/* [170s] PERCENT */
named!(pub percent<&str, &str>, verify!(
    take_s!(3),
    |val:&str| {
        let bytes = val.as_bytes();
        bytes[0] == 0x25 && is_hex_digit(bytes[1]) && is_hex_digit(bytes[2])
    }
));

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult;
    use nom::ErrorKind;

    #[test]
    fn langtag_normal_test() {
        let input    = "@en-a-bbb-a-ccc,rest";
        let expected = IResult::Done(",rest", Langtag{
            language: "en",
            subtags:  vec!["a", "bbb", "a", "ccc"]
        });
        assert_eq!(langtag(input), expected);
    }

    #[test]
    fn langtag_abnormal_test() {
        let input    = "@en01-a-bbb-a-ccc,rest";
        let expected = IResult::Error(ErrorKind::Tag);
        assert_eq!(langtag(input), expected);
    }

    #[test]
    fn integer_normal_test() {
        let input    = "-1234rest";
        let expected = IResult::Done("rest", Integer{
            sign:          Some("-"),
            integer_part:  "1234",
        });
        assert_eq!(integer(input), expected);
    }

    #[test]
    fn decimal_normal_test() {
        let input    = "-1234.5678rest";
        let expected = IResult::Done("rest", Decimal{
            sign:          Some("-"),
            integer_part:  Some("1234"),
            decimal_point: ".",
            decimal_part:  "5678",
        });
        assert_eq!(decimal(input), expected);
    }

    #[test]
    fn double_normal_test() {
        let input    = "-1234.5678e-0123456789rest";
        let expected = IResult::Done("rest", Double{
            sign:     Some("-"),
            mantissa: Mantissa::IntegerDecimal {
                integer_part:  "1234",
                decimal_point: ".",
                decimal_part:  Some("5678"),
            },
            exponent: exponent("e-0123456789").unwrap().1,
        });
        assert_eq!(double(input), expected);

        let input    = "-.5678e-0123456789rest";
        let expected = IResult::Done("rest", Double{
            sign:     Some("-"),
            mantissa: Mantissa::Decimal {
                decimal_point: ".",
                decimal_part:  "5678",
            },
            exponent: exponent("e-0123456789").unwrap().1,
        });
        assert_eq!(double(input), expected);

        let input    = "+5678e-0123456789rest";
        let expected = IResult::Done("rest", Double{
            sign:     Some("+"),
            mantissa: Mantissa::Integer {
                integer_part: "5678",
            },
            exponent: exponent("e-0123456789").unwrap().1,
        });
        assert_eq!(double(input), expected);
    }

    #[test]
    fn exponent_normal_test() {
        let input    = "e-0123456789rest";
        let expected = IResult::Done("rest", Exponent{
            exponent_symbol: "e",
            sign:            Some("-"),
            exponent_number: "0123456789",
        });
        assert_eq!(exponent(input), expected);

        let input    = "E+0123456789rest";
        let expected = IResult::Done("rest", Exponent{
            exponent_symbol: "E",
            sign:            Some("+"),
            exponent_number: "0123456789",
        });
        assert_eq!(exponent(input), expected);

        let input    = "e0123456789rest";
        let expected = IResult::Done("rest", Exponent{
            exponent_symbol: "e",
            sign:            None,
            exponent_number: "0123456789",
        });
        assert_eq!(exponent(input), expected);
    }

    #[test]
    fn ws_test() {
        assert_eq!(ws(" 	\r\nrest"), IResult::Done("rest", " 	\r\n"));
    }

    #[test]
    fn anon_test() {
        assert_eq!(anon("[ ]a"), IResult::Done("a", "[ ]"));
    }

    #[test]
    fn percent_test() {
        assert_eq!(percent("%2Abc"), IResult::Done("bc", "%2A"));
    }
}
