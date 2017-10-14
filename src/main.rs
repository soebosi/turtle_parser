#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod grammar;

use grammar::{TurtleParser, Rule};
use pest::Parser;
use std::{fs, env};
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = fs::File::open(&args[1]).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let pairs = TurtleParser::parse_str(Rule::turtle_doc, &contents).unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs.flatten() {
        let rule = format!("{:?}", pair.as_rule());
        let text = pair.as_str().replace("\n", "\\n");
        println!("{:<40}: {}", rule, text);
    }
}
