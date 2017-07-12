#[macro_use]
extern crate nom;

use std::str;
use nom::{
    IResult,
    ErrorKind,
    is_hex_digit
};

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

fn is_pn_local_esc(c: char) -> bool {
    match "_~.-!$&'()*+,;=/?#@%".find(c) {
        Some(_) => true,
        _       => false,
    }
}

fn is_ws(c: char) -> bool {
    let u = c as u32;
    u == 0x20 || u == 0x9 || u == 0xD || u == 0xA
}

fn is_anon(c: char) -> bool {
    c == '[' || c == ']' || is_ws(c)
}

/* [161s] WS */
named!(ws<&str, &str>, take_while_s!(is_ws));

/* [162s] ANON */
named!(anon<&str, &str>, verify!(
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

/* [163s] PN_CHARS_BASE */
named!(pn_chars_base<&str, &str>, verify!(
    take_s!(1),
    |val:&str| {
        if let Some(c) = val.chars().next() {
            return is_pn_chars_base(c)
        }
        false
    }
));

/* [164s] PN_CHARS_U */
named!(pn_chars_u<&str, &str>, verify!(
    take_s!(1),
    |val:&str| {
        if let Some(c) = val.chars().next() {
            return is_pn_chars_u(c)
        }
        false
    }
));

/* [166s] PN_CHARS */
named!(pn_chars<&str, &str>, verify!(
    take_s!(1),
    |val:&str| {
        if let Some(c) = val.chars().next() {
            return is_pn_chars(c)
        }
        false
    }
));

/* [167s] PN_PREFIX */
named!(pn_prefix<&str, &str>, verify!(
    take_until!(":"),
    |val:&str| {
        let len = val.char_indices().count();
        val.char_indices().all(|(idx, c)| {
            if idx == 0 {
                is_pn_chars_base(c)
            } else if idx == len - 1 {
                c != '.'
            } else {
                is_pn_chars(c) || c == '.'
            }
        })
    }
));

fn is_pn_local(c: char) -> bool {
    is_pn_chars(c) ||
    c == '.'  || c == ':'              ||
    c == '%'  || is_hex_digit(c as u8) ||
    c == '\\' || is_pn_local_esc(c)
}
/* [168s] PN_LOCAL */
named!(pn_local<&str, &str>, verify!(
    take_while_s!(is_pn_local),
    |val:&str| {
        let len = val.char_indices().count();
        let mut percent_count = 0;
        let mut pn_local_esc_count = 0;
        val.char_indices().all(|(idx, c)| {
            if percent_count > 0 {
                percent_count = percent_count - 1;
                is_hex_digit(c as u8)
            } else if c == '%' {
                percent_count = 2;
                true
            } else if pn_local_esc_count > 0 {
                pn_local_esc_count = pn_local_esc_count - 1;
                is_pn_local_esc(c)
            } else if c == '\\' {
                pn_local_esc_count = 1;
                true
            } else if idx == 0 {
                is_pn_chars_u(c) || c == ':'
            } else if idx == len - 1 {
                is_pn_chars(c) || c == ':'
            } else {
                is_pn_chars(c) || c == ':' || c == '.'
            }
        }) && percent_count == 0 && pn_local_esc_count == 0
    }
));

/* [169s] PLX */
named!(plx<&str, &str>, alt!(percent | pn_local_esc));

/* [170s] PERCENT */
named!(percent<&str, &str>, verify!(
    take_s!(3),
    |val:&str| {
        let bytes = val.as_bytes();
        bytes[0] == 0x25 && is_hex_digit(bytes[1]) && is_hex_digit(bytes[2])
    }
));

/* [172s]  PN_LOCAL_ESC */
named!(pn_local_esc<&str, &str>, verify!(
    take_s!(2),
    |val:&str| {
        let c1 = val.chars().nth(0).unwrap();
        let c2 = val.chars().nth(1).unwrap();
        c1 == '\\' && is_pn_local_esc(c2)
    }
));

fn main() {
    assert_eq!(ws("   a")                , IResult::Done("a", "   ")          );
    assert_eq!(anon("[ ]a")              , IResult::Done("a", "[ ]")          );
    assert_eq!(pn_chars_base("a%2Ab")    , IResult::Done("%2Ab", "a")         );
    assert_eq!(pn_chars_u("_a")          , IResult::Done("a"   , "_")         );
    assert_eq!(pn_chars("-_a")           , IResult::Done("_a"  , "-")         );
    assert_eq!(pn_chars(":a2Ab")         , IResult::Error(ErrorKind::Verify)  );
    assert_eq!(pn_prefix("hog.e:")       , IResult::Done(":", "hog.e")        );
    assert_eq!(pn_prefix("hoge.:")       , IResult::Error(ErrorKind::Verify)  );
    assert_eq!(pn_local("hoge:fuga piyo"), IResult::Done(" piyo", "hoge:fuga"));
    assert_eq!(pn_local("hoge:%28 piyo") , IResult::Done(" piyo", "hoge:%28") );
    assert_eq!(pn_local("hoge:%2 piyo")  , IResult::Error(ErrorKind::Verify)  );
    assert_eq!(pn_local("hoge:%__ piyo") , IResult::Error(ErrorKind::Verify)  );
    assert_eq!(pn_local("hoge:\\* piyo") , IResult::Done(" piyo", "hoge:\\*") );
    assert_eq!(pn_local("%28\\* piyo")   , IResult::Done(" piyo", "%28\\*")   );
    assert_eq!(pn_local("%28\\ piyo")    , IResult::Error(ErrorKind::Verify)  );
    assert_eq!(plx("%2Abc")              , IResult::Done("bc"   , "%2A")      );
    assert_eq!(plx("\\.%2A")             , IResult::Done("%2A"  , "\\.")      );
    assert_eq!(percent("%2Abc")          , IResult::Done("bc"   , "%2A")      );
    assert_eq!(pn_local_esc("\\.a b c")  , IResult::Done("a b c", "\\.")      );
}
