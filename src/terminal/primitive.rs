extern crate nom;

use std::str;
use nom::*;

fn is_ws(c: char) -> bool {
    let u = c as u32;
    u == 0x20 || u == 0x9 || u == 0xD || u == 0xA
}

fn is_anon(c: char) -> bool {
    c == '[' || c == ']' || is_ws(c)
}

fn is_exponent(c: char) -> bool {
    c == 'e' || c == 'E' || c == '+' || c == '-' || is_digit(c as u8)
}

named!(pub exponent<&str, &str>, verify!(
    take_while_s!(is_exponent),
    |val:&str| {
        let len = val.char_indices().count();
        val.char_indices().all(|(idx, c)| {
            if idx == 0 {
                c == 'e' || c == 'E'
            } else if idx == 1 {
                c == '+' || c == '-' || is_digit(c as u8)
            } else {
                is_digit(c as u8)
            }
        })
    }
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

    #[test]
    fn exponent_test() {
        assert_eq!(exponent("e-0001rest"), IResult::Done("rest", "e-0001"));
    }

    #[test]
    fn ws_test() {
        assert_eq!(ws("   a"), IResult::Done("a", "   "));
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
