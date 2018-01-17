extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate serde_derive;

mod grammar;

use grammar::{TurtleParser, Rule};
use pest::{Parser, iterators, inputs};
use std::{fs, env};
use std::io::Read;

#[derive(Serialize, Deserialize)]
struct Result {
    rule: String,
    text: Option<String>,
    children: Vec<Result>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = fs::File::open(&args[1]).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let pairs = TurtleParser::parse_str(Rule::turtle_doc, &contents)
        .unwrap_or_else(|e| panic!("{}", e));
    let mut result = Vec::new();
    create_result_from_pairs(pairs, &mut result);
    println!("{}", serde_json::to_string(&result).unwrap());
}

fn create_result_from_pairs(
    pairs: iterators::Pairs<grammar::Rule, inputs::StrInput>,
    result: &mut Vec<Result>,
) {
    for pair in pairs {
        let rule = format!("{:?}", pair.as_rule());
        let text = String::from(pair.as_str());
        let mut children = Vec::new();
        create_result_from_pairs(pair.into_inner(), &mut children);
        result.push(Result {
            rule,
            text: if children.len() == 0 {
                Some(text)
            } else {
                None
            },
            children,
        });
    }
}
