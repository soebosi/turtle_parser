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

#[derive(PartialEq, Debug)]
pub enum Plx<'a> {
    Percent(&'a str),
    PnLocalEsc(&'a str),
}
/* [169s] PLX */
named!(pub plx<&str, Plx>, alt!(
    map!(percent     , Plx::Percent   ) |
    map!(pn_local_esc, Plx::PnLocalEsc)
));

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult;

    #[test]
    fn plx_test() {
        assert_eq!(plx("%2Arest"), IResult::Done("rest", Plx::Percent("%2A")));
        assert_eq!(plx("\\.rest"), IResult::Done("rest", Plx::PnLocalEsc("\\.")));
    }
}
