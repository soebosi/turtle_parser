extern crate nom;

use std::str;
use nom::*;

fn is_echar(c: char) -> bool {
    c == '\\' || c == 't' || c == 'b' || c == 'n' || c == 'r' || c == 'f' || c == '"' || c == '\''
}
/* [26] UCHAR */
named!(uchar<&str, &str>, alt!(
    verify!(
        take_s!(6),
        |val:&str| {
            let len = val.char_indices().count();
            val.char_indices().all(|(idx, c)| {
                if idx == 0 {
                    c == '\\'
                } else if idx == 1 {
                    c == 'u'
                } else {
                    is_hex_digit(c as u8)
                }
            })
        }
    ) |
    verify!(
        take_s!(10),
        |val:&str| {
            let len = val.char_indices().count();
            val.char_indices().all(|(idx, c)| {
                if idx == 0 {
                    c == '\\'
                } else if idx == 1 {
                    c == 'U'
                } else {
                    is_hex_digit(c as u8)
                }
            })
        }
    )
));

/* [159s] ECHAR */
named!(echar<&str, &str>, verify!(
    take_s!(2),
    |val:&str| {
        let len = val.char_indices().count();
        val.char_indices().all(|(idx, c)| {
            if idx == 0 {
                c == '\\'
            } else {
                is_echar(c)
            }
        })
    }
));

named!(string_literal_single_quote_body_char<&str, &str>, verify!(
    take_s!(1),
    |val:&str| {
        if let Some(c) = val.chars().next() {
            let u = c as u32;
            match u {
                0x22 | 0x5C | 0xA | 0xD
                  => false,
                _ => true,
            }
        } else {
            false
        }
    }
));

named!(pub string_literal_single_quote_body<&str, Vec<&str>>, many0!(
    alt!(string_literal_single_quote_body_char | echar | uchar)
));
/* [25] STRING_LITERAL_LONG_QUOTE */
named!(pub string_literal_single_quote<&str, &str>, delimited!(
    tag!("'"),
    verify!(
        take_until_s!("'"),
        |val: &str| {
            match string_literal_single_quote_body(val) {
                IResult::Error(_) => false,
                _                 => true,
            }
        }
    ),
    tag!("'")
));

named!(pub string_literal_long_single_quote_body<&str, Vec<&str>>, many0!(
    alt!(tag!("'") | echar | uchar)
));
/* [24] STRING_LITERAL_LONG_SINGLE_QUOTE */
named!(pub string_literal_long_single_quote<&str, &str>, delimited!(
    tag!("'''"),
    verify!(
        take_until_s!("'''"),
        |val: &str| {
            match string_literal_long_single_quote_body(val) {
                IResult::Error(_) => false,
                _                 => true,
            }
        }
    ),
    tag!("'''")
));


named!(pub string_literal_long_quote_body<&str, Vec<&str>>, many0!(
    alt!(tag!("\"") | echar | uchar)
));
/* [25] STRING_LITERAL_LONG_QUOTE */
named!(pub string_literal_long_quote<&str, &str>, delimited!(
    tag!("\"\"\""),
    verify!(
        take_until_s!("\"\"\""),
        |val: &str| {
            match string_literal_long_quote_body(val) {
                IResult::Error(_) => false,
                _                 => true,
            }
        }
    ),
    tag!("\"\"\"")
));

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult;

    #[test]
    fn uchar_test() {
        assert_eq!(uchar("\\u02FFrest")    , IResult::Done("rest", "\\u02FF")    );
        assert_eq!(uchar("\\U02FFAABBrest"), IResult::Done("rest", "\\U02FFAABB"));
    }

    #[test]
    fn echar_test() {
        assert_eq!(echar("\\trest") , IResult::Done("rest", "\\t") );
        assert_eq!(echar("\\brest") , IResult::Done("rest", "\\b") );
        assert_eq!(echar("\\nrest") , IResult::Done("rest", "\\n") );
        assert_eq!(echar("\\rrest") , IResult::Done("rest", "\\r") );
        assert_eq!(echar("\\frest") , IResult::Done("rest", "\\f") );
        assert_eq!(echar("\\\"rest"), IResult::Done("rest", "\\\""));
        assert_eq!(echar("\\'rest") , IResult::Done("rest", "\\'") );
        assert_eq!(echar("\\\\rest"), IResult::Done("rest", "\\\\"));
    }

    #[test]
    fn string_literal_single_quote_test() {
        let input    = "'\\t\\u02FFhoge'rest";
        let expected = "\\t\\u02FFhoge";
        assert_eq!(string_literal_single_quote(input), IResult::Done("rest", expected));
    }

    #[test]
    fn string_literal_long_single_quote_test() {
        let input    = "'''\\t\\u02FF'''rest";
        let expected = "\\t\\u02FF";
        assert_eq!(string_literal_long_single_quote(input), IResult::Done("rest", expected));
    }

    #[test]
    fn string_literal_long_quote_test() {
        let input    = "\"\"\"\\t\\u02FF\"\"\"rest";
        let expected = "\\t\\u02FF";
        assert_eq!(string_literal_long_quote(input), IResult::Done("rest", expected));
    }
}
