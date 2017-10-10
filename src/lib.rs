#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;

#[cfg(debug_assertions)]
const _GRAMMER: &'static str = include_str!("grammer.pest");

#[derive(Parser)]
#[grammar = "grammer.pest"]
struct TurtleParser;

#[test]
fn iri_test() {
    parses_to! {
        parser: TurtleParser,
        input: "<http://www.example.com>",
        rule: Rule::iri,
        tokens: [
            iri(0, 24, [
                iriref(0, 24)
            ])
        ]
    };

    parses_to! {
        parser: TurtleParser,
        input: "foaf:Person",
        rule: Rule::iri,
        tokens: [
            iri(0, 11, [
                prefixed_name(0, 11, [
                    pname_ln(0, 11)
                ])
            ])
        ]
    };
}

#[test]
fn prefixd_name_test() {
    parses_to! {
        parser: TurtleParser,
        input: "foaf:",
        rule: Rule::prefixed_name,
        tokens: [
            prefixed_name(0, 5, [
                pname_ns(0, 5)
            ])
        ]
    };

    parses_to! {
        parser: TurtleParser,
        input: "foaf:Person",
        rule: Rule::prefixed_name,
        tokens: [
            prefixed_name(0, 11, [
                pname_ln(0, 11)
            ])
        ]
    };
}

#[test]
fn blank_node_test() {
    parses_to! {
        parser: TurtleParser,
        input: "_:blank",
        rule: Rule::blank_node,
        tokens: [
            blank_node(0, 7, [
                blank_node_label(0, 7)
            ])
        ]
    };

    parses_to! {
        parser: TurtleParser,
        input: "[     ]",
        rule: Rule::blank_node,
        tokens: [
            blank_node(0, 7, [
                anon(0, 7)
            ])
        ]
    };
}
