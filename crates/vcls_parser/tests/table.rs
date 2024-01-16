use vcls_ast::*;
use vcls_parser::parse;

#[test]
fn table() {
    assert_eq!(
        parse(
            r#"
table test {
"foo": "bar", // comment
{"integer"} /* comment */: 1 ,
"": "empty // key",
{"/* */
"}: {HEREDOC"
    multiline
"HEREDOC},
"rtime" // : 2m,
: 1m , "bool" : /* false */true,
"backend or acl" : backend_or_acl //,
}"#
        )
        .unwrap(),
        Vcl {
            declarations: vec![Declaration::Table(TableDeclaration {
                name: "test".to_string(),
                typ: Type::String,
                entries: vec![
                    TableEntry {
                        key: StringLiteral {
                            tokens: vec![StringToken {
                                value: "foo".to_string(),
                                span: Span(14, 19),
                            }],
                            span: Span(14, 19),
                        },
                        value: TableValue::Literal(Literal::String(StringLiteral {
                            tokens: vec![StringToken {
                                value: "bar".to_string(),
                                span: Span(21, 26),
                            }],
                            span: Span(21, 26),
                        })),
                        span: Span(14, 26),
                    },
                    TableEntry {
                        key: StringLiteral {
                            tokens: vec![StringToken {
                                value: "integer".to_string(),
                                span: Span(39, 50),
                            }],
                            span: Span(39, 64),
                        },
                        value: TableValue::Literal(Literal::Integer(IntegerLiteral {
                            value: 1,
                            span: Span(66, 67),
                        })),
                        span: Span(39, 67),
                    },
                    TableEntry {
                        key: StringLiteral {
                            tokens: vec![StringToken {
                                value: "".to_string(),
                                span: Span(70, 72),
                            }],
                            span: Span(70, 72),
                        },
                        value: TableValue::Literal(Literal::String(StringLiteral {
                            tokens: vec![StringToken {
                                value: "empty // key".to_string(),
                                span: Span(74, 88),
                            }],
                            span: Span(74, 88),
                        })),
                        span: Span(70, 88),
                    },
                    TableEntry {
                        key: StringLiteral {
                            tokens: vec![StringToken {
                                value: "/* */\n".to_string(),
                                span: Span(90, 100),
                            }],
                            span: Span(90, 100),
                        },
                        value: TableValue::Literal(Literal::String(StringLiteral {
                            tokens: vec![StringToken {
                                value: "\n    multiline\n".to_string(),
                                span: Span(102, 135),
                            }],
                            span: Span(102, 135),
                        })),
                        span: Span(90, 135),
                    },
                    TableEntry {
                        key: StringLiteral {
                            tokens: vec![StringToken {
                                value: "rtime".to_string(),
                                span: Span(137, 144),
                            }],
                            span: Span(137, 154),
                        },
                        value: TableValue::Literal(Literal::RTime(RTimeLiteral {
                            value: RelativeTime::from_min(1.0),
                            span: Span(156, 158),
                        })),
                        span: Span(137, 158),
                    },
                    TableEntry {
                        key: StringLiteral {
                            tokens: vec![StringToken {
                                value: "bool".to_string(),
                                span: Span(161, 167),
                            }],
                            span: Span(161, 168),
                        },
                        value: TableValue::Literal(Literal::Bool(BoolLiteral {
                            value: true,
                            span: Span(181, 185),
                        })),
                        span: Span(161, 185),
                    },
                    TableEntry {
                        key: StringLiteral {
                            tokens: vec![StringToken {
                                value: "backend or acl".to_string(),
                                span: Span(187, 203),
                            }],
                            span: Span(187, 204),
                        },
                        value: TableValue::Ident(Variable {
                            name: "backend_or_acl".to_string(),
                            properties: vec![],
                            sub_field: None,
                            span: Span(206, 220),
                        }),
                        span: Span(187, 220),
                    },
                ],
                span: Span(1, 226),
            })],
        },
    );
}
