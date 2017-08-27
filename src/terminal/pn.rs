extern crate nom;

use std::str;
use nom::*;

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

fn is_blank_node_label(c: char) -> bool {
    is_pn_chars(c) || c == '.'
}

named!(pub pname_ns<&str, &str>, do_parse!(
    prefix: pn_prefix >>
    tag!(":") >>
    (prefix)
));

/* [141s] BLANK_NODE_LABEL */
named!(pub blank_node_label<&str, &str>, do_parse!(
    tag!("_:") >>
    node: verify!(
      take_while_s!(is_blank_node_label),
      |val:&str| {
          true
      }
    ) >>
    (node)
));

/* [163s] PN_CHARS_BASE */
named!(pub pn_chars_base<&str, &str>, verify!(
    take_s!(1),
    |val:&str| {
        if let Some(c) = val.chars().next() {
            return is_pn_chars_base(c)
        }
        false
    }
));

/* [164s] PN_CHARS_U */
named!(pub pn_chars_u<&str, &str>, verify!(
    take_s!(1),
    |val:&str| {
        if let Some(c) = val.chars().next() {
            return is_pn_chars_u(c)
        }
        false
    }
));

/* [166s] PN_CHARS */
named!(pub pn_chars<&str, &str>, verify!(
    take_s!(1),
    |val:&str| {
        if let Some(c) = val.chars().next() {
            return is_pn_chars(c)
        }
        false
    }
));

/* [167s] PN_PREFIX */
named!(pub pn_prefix<&str, &str>, verify!(
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
named!(pub pn_local<&str, &str>, verify!(
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

/* [172s]  PN_LOCAL_ESC */
named!(pub pn_local_esc<&str, &str>, verify!(
    take_s!(2),
    |val:&str| {
        let c1 = val.chars().nth(0).unwrap();
        let c2 = val.chars().nth(1).unwrap();
        c1 == '\\' && is_pn_local_esc(c2)
    }
));

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult;

    #[test]
    fn pname_ns_test() {
        let input    = "test:rest";
        let expected = IResult::Done("rest", "test");
        assert_eq!(pname_ns(input), expected);

        let input    = ":rest";
        let expected = IResult::Done("rest", "");
        assert_eq!(pname_ns(input), expected);
    }

    #[test]
    fn blank_node_label_test() {
        let input    = "_:test_rest";
        let expected = IResult::Done("_rest", "test");
        assert_eq!(blank_node_label(input), expected);
    }

    #[test]
    fn pn_chars_base_test() {
        assert_eq!(pn_chars_base("a%2Ab"), IResult::Done("%2Ab", "a"));
    }

    #[test]
    fn pn_chars_u_test() {
        assert_eq!(pn_chars_u("_a"), IResult::Done("a", "_"));
    }

    #[test]
    fn pn_chars_test() {
        assert_eq!(pn_chars("-_a")  , IResult::Done("_a", "-")         );
        assert_eq!(pn_chars(":a2Ab"), IResult::Error(ErrorKind::Verify));
    }

    #[test]
    fn pn_prefix_test() {
        assert_eq!(pn_prefix("hog.e:"), IResult::Done(":", "hog.e")      );
        assert_eq!(pn_prefix("hoge.:"), IResult::Error(ErrorKind::Verify));
    }

    #[test]
    fn pn_local_test() {
        assert_eq!(pn_local("hoge:fuga piyo"), IResult::Done(" piyo", "hoge:fuga"));
        assert_eq!(pn_local("hoge:%28 piyo") , IResult::Done(" piyo", "hoge:%28") );
        assert_eq!(pn_local("hoge:%2 piyo")  , IResult::Error(ErrorKind::Verify)  );
        assert_eq!(pn_local("hoge:%__ piyo") , IResult::Error(ErrorKind::Verify)  );
        assert_eq!(pn_local("hoge:\\* piyo") , IResult::Done(" piyo", "hoge:\\*") );
        assert_eq!(pn_local("%28\\* piyo")   , IResult::Done(" piyo", "%28\\*")   );
        assert_eq!(pn_local("%28\\ piyo")    , IResult::Error(ErrorKind::Verify)  );
    }

    #[test]
    fn pn_local_esc_test() {
        assert_eq!(pn_local_esc("\\.a b c"), IResult::Done("a b c", "\\."));
    }
}
