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
fn blank_node_property_list_test() {
    parses_to! {
        parser: TurtleParser,
        input: "[<http://verb> <http://object1>, <http://object2>; a <http://object3>]",
        rule: Rule::blank_node_property_list,
        tokens: [
            blank_node_property_list(0, 70, [
                predicate_object_list(1, 69, [
                    verb(1, 14, [ predicate(1, 14, [ iri(1, 14, [ iriref(1, 14)]) ]) ]),
                    object_list(15, 49, [
                        object(15, 31, [ iri(15, 31, [ iriref(15, 31)]) ]),
                        object(33, 49, [ iri(33, 49, [ iriref(33, 49)]) ])
                    ]),
                    verb(51, 52),
                    object_list(53, 69, [
                        object(53, 69, [ iri(53, 69, [ iriref(53, 69)]) ])
                    ])
                ])
            ])
        ]
    };
}

#[test]
fn collection_test() {
    parses_to! {
        parser: TurtleParser,
        input: "(<http://www.example.com> ([ ]))",
        rule: Rule::collection,
        tokens: [
            collection(0, 32, [
                object(1, 25, [
                    iri(1, 25, [ iriref(1, 25) ])
                ]),
                object(26, 31, [
                    collection(26, 31, [
                        object(27, 30, [ blank_node(27, 30, [ anon(27, 30) ]) ])
                    ])
                ])
            ])
        ]
    };
}

#[test]
fn numeric_literal_test() {
    parses_to! {
        parser: TurtleParser,
        input: "+10",
        rule: Rule::numeric_literal,
        tokens: [
            numeric_literal(0, 3, [
                integer(0, 3)
            ])
        ]
    };

    parses_to! {
        parser: TurtleParser,
        input: "+1.0",
        rule: Rule::numeric_literal,
        tokens: [
            numeric_literal(0, 4, [
                decimal(0, 4)
            ])
        ]
    };

    parses_to! {
        parser: TurtleParser,
        input: "+1.0e-9",
        rule: Rule::numeric_literal,
        tokens: [
            numeric_literal(0, 7, [
                double(0, 7)
            ])
        ]
    };
}

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
