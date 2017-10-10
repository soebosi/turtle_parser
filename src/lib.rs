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
fn rdf_literal_test() {
    parses_to! {
        parser: TurtleParser,
        input: r#""test"@en"#,
        rule: Rule::rdf_literal,
        tokens: [
            rdf_literal(0, 9, [
                string(0, 6, [
                    string_literal_quote(0, 6)
                ]),
                langtag(6, 9)
            ])
        ]
    };

    parses_to! {
        parser: TurtleParser,
        input: r#""test"^^<http://www.example.com>"#,
        rule: Rule::rdf_literal,
        tokens: [
            rdf_literal(0, 32, [
                string(0, 6, [
                    string_literal_quote(0, 6)
                ]),
                iri(8, 32, [
                    iriref(8, 32)
                ])
            ])
        ]
    };
}

#[test]
fn boolean_literal_test() {
    parses_to! {
        parser: TurtleParser,
        input: "true",
        rule: Rule::boolean_literal,
        tokens: [
            boolean_literal(0, 4)
        ]
    };

    parses_to! {
        parser: TurtleParser,
        input: "false",
        rule: Rule::boolean_literal,
        tokens: [
            boolean_literal(0, 5)
        ]
    };
}

#[test]
fn string_test() {
    parses_to! {
        parser: TurtleParser,
        input: r#""test""#,
        rule: Rule::string,
        tokens: [
            string(0, 6, [
                string_literal_quote(0, 6)
            ])
        ]
    };

    parses_to! {
        parser: TurtleParser,
        input: r#"'test'"#,
        rule: Rule::string,
        tokens: [
            string(0, 6, [
                string_literal_single_quote(0, 6)
            ])
        ]
    };

    parses_to! {
        parser: TurtleParser,
        input: r#""""test"test""""#,
        rule: Rule::string,
        tokens: [
            string(0, 15, [
                string_literal_long_quote(0, 15)
            ])
        ]
    };

    parses_to! {
        parser: TurtleParser,
        input: r#"'''test'test'''"#,
        rule: Rule::string,
        tokens: [
            string(0, 15, [
                string_literal_long_single_quote(0, 15)
            ])
        ]
    };
}

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
