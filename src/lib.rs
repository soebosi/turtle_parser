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
fn turtle_doc_test() {
    parses_to! {
        parser: TurtleParser,
        input: "@base <http://example.org/> .
                @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
                @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
                @prefix foaf: <http://xmlns.com/foaf/0.1/> .
                @prefix rel: <http://www.perceive.net/schemas/relationship/> .

                <#green-goblin>
                    rel:enemyOf <#spiderman> ;
                    a foaf:Person ;    # in the context of the Marvel universe
                    foaf:name \"Green Goblin\" .
                <#spiderman>
                    rel:enemyOf <#green-goblin> ;
                    a foaf:Person ;
                    foaf:name \"Spiderman\", \"Человек-паук\"@ru .",
        rule: Rule::turtle_doc,
        tokens: [
            turtle_doc(0, 713, [
                statement(0, 29, [
                    directive(0, 29, [
                        base(0, 29, [
                            iriref(6, 27)
                        ])
                    ])
                ]),
                statement(46, 106, [
                    directive(46, 106, [
                        prefix_id(46, 106, [
                            pname_ns(54, 58),
                            iriref(59, 104)
                        ])
                    ])
                ]),
                statement(123, 178, [
                    directive(123, 178, [
                        prefix_id(123, 178, [
                            pname_ns(131, 136),
                            iriref(137, 176)
                        ])
                    ])
                ]),
                statement(195, 239, [
                    directive(195, 239, [
                        prefix_id(195, 239, [
                            pname_ns(203, 208),
                            iriref(209, 237)
                        ])
                    ])
                ]),
                statement(256, 318, [
                    directive(256, 318, [
                        prefix_id(256, 318, [
                            pname_ns(264, 268),
                            iriref(269, 316)
                        ])
                    ])
                ]),
                statement(336, 524, [
                    triples(336, 523, [
                        subject(336, 351, [
                            iri(336, 351, [
                                iriref(336, 351)
                            ])
                        ]),
                        predicate_object_list(372, 523, [
                            verb(372, 383, [
                                predicate(372, 383, [
                                    iri(372, 383, [
                                        prefixed_name(372, 383, [
                                            pname_ln(372, 383)
                                        ])
                                    ])
                                ])
                            ]),
                            object_list(384, 397, [
                                object(384, 396, [
                                    iri(384, 396, [
                                        iriref(384, 396)
                                    ])
                                ])
                            ]),
                            verb(419, 420),
                            object_list(421, 433, [
                                object(421, 432, [
                                    iri(421, 432, [
                                        prefixed_name(421, 432, [
                                            pname_ln(421, 432)
                                        ])
                                    ])
                                ])
                            ]),
                            verb(498, 507, [
                                predicate(498, 507, [
                                    iri(498, 507, [
                                        prefixed_name(498, 507, [
                                            pname_ln(498, 507)
                                        ])
                                    ])
                                ])
                            ]),
                            object_list(508, 523, [
                                object(508, 523, [
                                    literal(508, 523, [
                                        rdf_literal(508, 523, [
                                            string(508, 522, [
                                                string_literal_quote(508, 522)
                                            ])
                                        ])
                                    ])
                                ])
                            ])
                        ])
                    ])
                ]),
                statement(541, 713, [
                    triples(541, 711, [
                        subject(541, 553, [
                            iri(541, 553, [
                                iriref(541, 553)
                            ])
                        ]),
                        predicate_object_list(574, 711, [
                            verb(574, 585, [
                                predicate(574, 585, [
                                    iri(574, 585, [
                                        prefixed_name(574, 585, [
                                            pname_ln(574, 585)
                                        ])
                                    ])
                                ])
                            ]),
                            object_list(586, 602, [
                                object(586, 601, [
                                    iri(586, 601, [
                                        iriref(586, 601)
                                    ])
                                ])
                            ]),
                            verb(624, 625),
                            object_list(626, 638, [
                                object(626, 637, [
                                    iri(626, 637, [
                                        prefixed_name(626, 637, [
                                            pname_ln(626, 637)
                                        ])
                                    ])
                                ])
                            ]),
                            verb(660, 669, [
                                predicate(660, 669, [
                                    iri(660, 669, [
                                        prefixed_name(660, 669, [
                                            pname_ln(660, 669)
                                        ])
                                    ])
                                ])
                            ]),
                            object_list(670, 711, [
                                object(670, 681, [
                                    literal(670, 681, [
                                        rdf_literal(670, 681, [
                                            string(670, 681, [
                                                string_literal_quote(670, 681)
                                            ])
                                        ])
                                    ])
                                ]),
                                object(683, 711, [
                                    literal(683, 711, [
                                        rdf_literal(683, 711, [
                                            string(683, 708, [
                                                string_literal_quote(683, 708)
                                            ]),
                                            langtag(708, 711)
                                        ])
                                    ])
                                ])
                            ])
                        ])
                    ])
                ])
            ])
        ]
    };
}

#[test]
fn verb_test() {
    parses_to! {
        parser: TurtleParser,
        input: "<http://www.example.com>",
        rule: Rule::verb,
        tokens: [
            verb(0, 24, [
                predicate(0, 24, [
                    iri(0, 24, [
                        iriref(0, 24)
                    ])
                ])
            ])
        ]
    };
}

#[test]
fn subject_test() {
    parses_to! {
        parser: TurtleParser,
        input: "<http://www.example.com>",
        rule: Rule::subject,
        tokens: [
            subject(0, 24, [
                iri(0, 24, [
                    iriref(0, 24)
                ])
            ])
        ]
    };

    parses_to! {
        parser: TurtleParser,
        input: "_:blank",
        rule: Rule::subject,
        tokens: [
            subject(0, 7, [
                blank_node(0, 7, [
                    blank_node_label(0, 7)
                ])
            ])
        ]
    };

    parses_to! {
        parser: TurtleParser,
        input: "(<http://www.example.com> ([ ]))",
        rule: Rule::subject,
        tokens: [
            subject(0, 32, [
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
            ])
        ]
    };
}

#[test]
fn predicate_test() {
    parses_to! {
        parser: TurtleParser,
        input: "<http://www.example.com>",
        rule: Rule::predicate,
        tokens: [
            predicate(0, 24, [
                iri(0, 24, [
                    iriref(0, 24)
                ])
            ])
        ]
    };
}

#[test]
fn object_test() {
    parses_to! {
        parser: TurtleParser,
        input: "<http://www.example.com>",
        rule: Rule::object,
        tokens: [
            object(0, 24, [
                iri(0, 24, [
                    iriref(0, 24)
                ])
            ])
        ]
    };

    parses_to! {
        parser: TurtleParser,
        input: "_:blank",
        rule: Rule::object,
        tokens: [
            object(0, 7, [
                blank_node(0, 7, [
                    blank_node_label(0, 7)
                ])
            ])
        ]
    };

    parses_to! {
        parser: TurtleParser,
        input: "(<http://www.example.com> ([ ]))",
        rule: Rule::object,
        tokens: [
            object(0, 32, [
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
            ])
        ]
    };

    parses_to! {
        parser: TurtleParser,
        input: "[<http://verb> <http://object1>, <http://object2>; a <http://object3>]",
        rule: Rule::object,
        tokens: [
            object(0, 70, [
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
            ])
        ]
    };

    parses_to! {
        parser: TurtleParser,
        input: r#""test"@en"#,
        rule: Rule::object,
        tokens: [
            object(0, 9, [
                literal(0, 9, [
                    rdf_literal(0, 9, [
                        string(0, 6, [
                            string_literal_quote(0, 6)
                        ]),
                        langtag(6, 9)
                    ])
                ])
            ])
        ]
    };
}

#[test]
fn literal_test() {
    parses_to! {
        parser: TurtleParser,
        input: r#""test"@en"#,
        rule: Rule::literal,
        tokens: [
            literal(0, 9, [
                rdf_literal(0, 9, [
                    string(0, 6, [
                        string_literal_quote(0, 6)
                    ]),
                    langtag(6, 9)
                ])
            ])
        ]
    };

    parses_to! {
        parser: TurtleParser,
        input: "+10",
        rule: Rule::literal,
        tokens: [
            literal(0, 3, [
                numeric_literal(0, 3, [
                    integer(0, 3)
                ])
            ])
        ]
    };

    parses_to! {
        parser: TurtleParser,
        input: "true",
        rule: Rule::literal,
        tokens: [
            literal(0, 4, [
                boolean_literal(0, 4)
            ])
        ]
    };
}

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
