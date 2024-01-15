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
                        key: "foo".to_string(),
                        value: TableValue::Literal(Literal::String(StringLiteral {
                            value: "bar".to_string(),
                            span: Span(21, 26),
                        })),
                        span: Span(14, 26),
                    },
                    TableEntry {
                        key: "integer".to_string(),
                        value: TableValue::Literal(Literal::Integer(IntegerLiteral {
                            value: 1,
                            span: Span(66, 67),
                        })),
                        span: Span(39, 67),
                    },
                    TableEntry {
                        key: "".to_string(),
                        value: TableValue::Literal(Literal::String(StringLiteral {
                            value: "empty // key".to_string(),
                            span: Span(74, 88),
                        })),
                        span: Span(70, 88),
                    },
                    TableEntry {
                        key: "/* */\n".to_string(),
                        value: TableValue::Literal(Literal::String(StringLiteral {
                            value: "\n    multiline\n".to_string(),
                            span: Span(102, 135),
                        })),
                        span: Span(90, 135),
                    },
                    TableEntry {
                        key: "rtime".to_string(),
                        value: TableValue::Literal(Literal::RTime(RTimeLiteral {
                            value: RelativeTime::from_min(1.0),
                            span: Span(156, 158),
                        })),
                        span: Span(137, 158),
                    },
                    TableEntry {
                        key: "bool".to_string(),
                        value: TableValue::Literal(Literal::Bool(BoolLiteral {
                            value: true,
                            span: Span(181, 185),
                        })),
                        span: Span(161, 185),
                    },
                    TableEntry {
                        key: "backend or acl".to_string(),
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
            })]
        }
    );
}
