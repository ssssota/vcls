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
                        value: TableValue::Literal(Literal::String("bar".to_string()))
                    },
                    TableEntry {
                        key: "integer".to_string(),
                        value: TableValue::Literal(Literal::Integer(1))
                    },
                    TableEntry {
                        key: "".to_string(),
                        value: TableValue::Literal(Literal::String("empty // key".to_string()))
                    },
                    TableEntry {
                        key: "/* */\n".to_string(),
                        value: TableValue::Literal(Literal::String(
                            "\n    multiline\n".to_string()
                        ))
                    },
                    TableEntry {
                        key: "rtime".to_string(),
                        value: TableValue::Literal(Literal::RTime(RelativeTime::from_min(1.0)))
                    },
                    TableEntry {
                        key: "bool".to_string(),
                        value: TableValue::Literal(Literal::Bool(true))
                    },
                    TableEntry {
                        key: "backend or acl".to_string(),
                        value: TableValue::Ident(Variable {
                            name: "backend_or_acl".to_string(),
                            properties: vec![],
                            sub_field: None
                        })
                    }
                ],
            })]
        }
    );
}
