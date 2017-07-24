#[macro_use]
extern crate nom;

use std::str;

pub mod terminal {
    pub mod pn;
    pub mod literal;
    pub mod primitive;
}
use terminal::pn;
use terminal::primitive;

#[derive(PartialEq, Debug)]
pub enum Plx<'a> {
    Percent(&'a str),
    PnLocalEsc(&'a str),
}
/* [169s] PLX */
named!(pub plx<&str, Plx>, alt!(
    map!(primitive::percent, Plx::Percent   ) |
    map!(pn::pn_local_esc  , Plx::PnLocalEsc)
));

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult;

    #[test]
    fn plx_test() {
        assert_eq!(plx("%2Arest"), IResult::Done("rest", Plx::Percent("2A")));
        assert_eq!(plx("\\.rest"), IResult::Done("rest", Plx::PnLocalEsc("\\.")));
    }
}
