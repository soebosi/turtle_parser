#[macro_use]
extern crate nom;

use std::str;

pub mod terminal {
    pub mod pn;
    pub mod literal;
    pub mod primitive;
}
use terminal::pn::pn_local_esc;
use terminal::primitive::percent;

/* [169s] PLX */
named!(pub plx<&str, &str>, alt!(percent | pn_local_esc));

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult;

    #[test]
    fn plx_test() {
        assert_eq!(plx("%2Arest"), IResult::Done("rest", "%2A"));
        assert_eq!(plx("\\.rest"), IResult::Done("rest", "\\."));
    }
}
