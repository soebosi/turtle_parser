extern crate nom;

use std::str;
use nom::*;

mod terminal {
    pub mod pn;
    pub mod literal;
    pub mod primitive;
}
use terminal::pn::pn_local_esc;
use terminal::primitive::percent;

/* [169s] PLX */
named!(plx<&str, &str>, alt!(percent | pn_local_esc));

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult;

    #[test]
    fn plx_test() {
        assert_eq!(plx("%2Abc") , IResult::Done("bc" , "%2A"));
        assert_eq!(plx("\\.%2A"), IResult::Done("%2A", "\\."));
    }
}
