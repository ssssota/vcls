use error::ParseError;
use pest::Parser;
use vcls_ast::*;

mod declaration;
mod error;
mod expression;
mod literal;
mod statement;
mod utils;
mod variable;

#[derive(pest_derive::Parser)]
#[grammar = "./fastly-vcl.pest"]
struct VclParser;

pub type ParseResult<T> = Result<T, Vec<error::ParseError>>;

pub fn parse(src: &str) -> ParseResult<Vcl> {
    let mut errors = vec![];
    let pairs = VclParser::parse(Rule::Vcl, src).map_err(|e| {
        vec![ParseError {
            message: e.to_string(),
        }]
    })?;
    let mut declarations = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::IncludeDeclaration => match declaration::include::handle(pair) {
                Ok(decl) => declarations.push(decl),
                Err(e) => errors.extend(e),
            },
            Rule::ImportDeclaration => match declaration::import::handle(pair) {
                Ok(decl) => declarations.push(decl),
                Err(e) => errors.extend(e),
            },
            Rule::AclDeclaration => match declaration::acl::handle(pair) {
                Ok(decl) => declarations.push(decl),
                Err(e) => errors.extend(e),
            },
            Rule::PenaltyboxDeclaration => match declaration::penaltybox::handle(pair) {
                Ok(decl) => declarations.push(decl),
                Err(e) => errors.extend(e),
            },
            Rule::RateCounterDeclaration => match declaration::ratecounter::handle(pair) {
                Ok(decl) => declarations.push(decl),
                Err(e) => errors.extend(e),
            },
            Rule::TableDeclaration => match declaration::table::handle(pair) {
                Ok(decl) => declarations.push(decl),
                Err(e) => errors.extend(e),
            },
            Rule::BackendDeclaration => match declaration::backend::handle(pair) {
                Ok(decl) => declarations.push(decl),
                Err(e) => errors.extend(e),
            },
            Rule::SubDeclaration => match declaration::sub::handle(pair) {
                Ok(decl) => declarations.push(decl),
                Err(e) => errors.extend(e),
            },
            Rule::EOI => {}
            _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }
    Ok(Vcl { declarations })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank() {
        assert_eq!(
            parse("").unwrap(),
            Vcl {
                declarations: vec![]
            }
        );
    }

    #[test]
    fn include() {
        assert_eq!(
            parse("include \"foo.vcl\";").unwrap(),
            Vcl {
                declarations: vec![Declaration::Include(IncludeDeclaration {
                    path: "foo.vcl".to_string()
                })]
            }
        );
    }

    #[test]
    fn import() {
        assert_eq!(
            parse("import foo;").unwrap(),
            Vcl {
                declarations: vec![Declaration::Import(ImportDeclaration {
                    ident: "foo".to_string()
                })]
            }
        );
    }

    #[test]
    fn acl() {
        assert_eq!(
            parse(
                "\
acl office_ip_ranges {
    \"localhost\";                                 # loopback
    \"192.0.2.0\"/24;                              # internal office...
    ! \"192.0.2.12\";                              # ... except for the vending machine
    \"198.51.100.4\";                              # remote VPN office
    \"2001:db8:ffff:ffff:ffff:ffff:ffff:ffff\";    # ipv6 address remote
}",
            )
            .unwrap(),
            Vcl {
                declarations: vec![Declaration::Acl(AclDeclaration {
                    name: "office_ip_ranges".to_string(),
                    entries: vec![
                        AclEntry {
                            negated: false,
                            addr: "localhost".to_string(),
                            cidr: 0,
                        },
                        AclEntry {
                            negated: false,
                            addr: "192.0.2.0".to_string(),
                            cidr: 24,
                        },
                        AclEntry {
                            negated: true,
                            addr: "192.0.2.12".to_string(),
                            cidr: 0,
                        },
                        AclEntry {
                            negated: false,
                            addr: "198.51.100.4".to_string(),
                            cidr: 0,
                        },
                        AclEntry {
                            negated: false,
                            addr: "2001:db8:ffff:ffff:ffff:ffff:ffff:ffff".to_string(),
                            cidr: 0,
                        }
                    ],
                })]
            }
        );
    }

    #[test]
    fn penaltybox() {
        assert_eq!(
            parse("penaltybox test { }").unwrap(),
            Vcl {
                declarations: vec![Declaration::PenaltyBox(PenaltyBoxDeclaration {
                    name: "test".to_string()
                })]
            }
        );
    }

    #[test]
    fn ratecounter() {
        assert_eq!(
            parse("ratecounter test { }").unwrap(),
            Vcl {
                declarations: vec![Declaration::RateCounter(RateCounterDeclaration {
                    name: "test".to_string()
                })]
            }
        );
    }

    #[test]
    fn table() {
        assert_eq!(
            parse(
                "\
table test {
    \"foo\": \"bar\", // comment
    {\"integer\"} /* comment */: 1 ,
    \"\": \"empty // key\",
    {\"/* */
\"}: {HEREDOC\"
    multiline
\"HEREDOC},
    \"rtime\" // : 2m,
: 1m , \"bool\" : /* false */true,
\"backend or acl\" : backend_or_acl //,
}"
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

    #[test]
    fn backend() {
        assert_eq!(
            parse(
                "
backend backend_name {

  # Required to be set for all VCL defined backends
  .dynamic = true;

  # Server location
  .host = \"storage.googleapis.com\";
  .ssl = true;
  .ssl_check_cert = always;

  # Timeouts and limits
  .connect_timeout = 1s;
  .max_connections = 200;

  # Health check
  .probe = {
    .dummy = false; # Boolean value determines the behavior of the probe.
                    # `true` performs DNS lookups only.
                    # `false` performs DNS lookups and HTTP health checks.
    .request = \"HEAD / HTTP/1.1\"  \"Host: storage.googleapis.com\" \"Connection: close\";
    .expected_response = 200;
    .interval = 60s;   # Send a check every 60s
    .timeout = 2s;     # Allow up to 2s for the backend to respond to the check
    .window = 5;       # Keep a history of 5 checks
    .initial = 4;      # Start with 4 successful checks in the history
    .threshold = 4;    # 4 of the recent checks must be successful for backend to be healthy
  }
}"
            )
            .unwrap(),
            Vcl {
                declarations: vec![Declaration::Backend(BackendDeclaration {
                    name: "backend_name".to_string(),
                    config: Object {
                        entries: vec![
                            (
                                "dynamic".to_string(),
                                ObjectValue::Literal(Literal::Bool(true))
                            ),
                            (
                                "host".to_string(),
                                ObjectValue::Literal(Literal::String(
                                    "storage.googleapis.com".to_string()
                                ))
                            ),
                            ("ssl".to_string(), ObjectValue::Literal(Literal::Bool(true))),
                            (
                                "ssl_check_cert".to_string(),
                                ObjectValue::Ident("always".to_string())
                            ),
                            (
                                "connect_timeout".to_string(),
                                ObjectValue::Literal(Literal::RTime(RelativeTime::from_sec(1.0)))
                            ),
                            (
                                "max_connections".to_string(),
                                ObjectValue::Literal(Literal::Integer(200))
                            ),
                            (
                                "probe".to_string(),
                                ObjectValue::Literal(
                                    Literal::Object(Object { entries: vec![
                                        (
                                            "dummy".to_string(),
                                            ObjectValue::Literal(Literal::Bool(false))
                                        ),
                                        (
                                            "request".to_string(),
                                            ObjectValue::Literal(Literal::String(
                                                "HEAD / HTTP/1.1Host: storage.googleapis.comConnection: close"
                                                    .to_string()
                                            ))
                                        ),
                                        (
                                            "expected_response".to_string(),
                                            ObjectValue::Literal(Literal::Integer(200))
                                        ),
                                        (
                                            "interval".to_string(),
                                            ObjectValue::Literal(Literal::RTime(
                                                RelativeTime::from_sec(60.0)
                                            ))
                                        ),
                                        (
                                            "timeout".to_string(),
                                            ObjectValue::Literal(Literal::RTime(
                                                RelativeTime::from_sec(2.0)
                                            ))
                                        ),
                                        (
                                            "window".to_string(),
                                            ObjectValue::Literal(Literal::Integer(5))
                                        ),
                                        (
                                            "initial".to_string(),
                                            ObjectValue::Literal(Literal::Integer(4))
                                        ),
                                        (
                                            "threshold".to_string(),
                                            ObjectValue::Literal(Literal::Integer(4))
                                        )
                                    ] })
                                )
                            )
                        ]
                    }
                })]
            }
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            parse("sub foo { return /* noop */; }").unwrap(),
            Vcl {
                declarations: vec![Declaration::Subroutine(SubroutineDeclaration {
                    name: "foo".to_string(),
                    return_type: Type::Void,
                    body: vec![Statement::Return(ReturnStatement { value: None })]
                })]
            }
        );
        assert_eq!(
            parse("sub foo STRING { return \"foo\" with_variable.bar:baz; }").unwrap(),
            Vcl {
                declarations: vec![Declaration::Subroutine(SubroutineDeclaration {
                    name: "foo".to_string(),
                    return_type: Type::String,
                    body: vec![Statement::Return(ReturnStatement {
                        value: Some(Expression::Binary(BinaryExpression {
                            lhs: Box::new(Expression::Literal(Literal::String("foo".to_string()))),
                            operator: BinaryOperator::Add,
                            rhs: Box::new(Expression::Variable(Variable {
                                name: "with_variable".to_string(),
                                properties: vec!["bar".to_string()],
                                sub_field: Some("baz".to_string())
                            }))
                        }))
                    })]
                })]
            }
        );
        assert_eq!(
            parse(
                "sub vcl_error {
                    synthetic.base64 \"dGVzdA==\";
                    synthetic \"foo\";
                }"
            )
            .unwrap(),
            Vcl {
                declarations: vec![Declaration::Subroutine(SubroutineDeclaration {
                    name: "vcl_error".to_string(),
                    return_type: Type::Void,
                    body: vec![
                        Statement::Synthetic(SyntheticStatement {
                            base64: true,
                            value: Expression::Literal(Literal::String("dGVzdA==".to_string()))
                        }),
                        Statement::Synthetic(SyntheticStatement {
                            base64: false,
                            value: Expression::Literal(Literal::String("foo".to_string()))
                        })
                    ]
                })]
            }
        );
        assert_eq!(
            parse(
                "sub vcl_recv {
                    if (req.http.host == \"www.example.com\") {
                        set req.backend_hint = example_com;
                    }
                }"
            )
            .unwrap(),
            Vcl {
                declarations: vec![Declaration::Subroutine(SubroutineDeclaration {
                    name: "vcl_recv".to_string(),
                    return_type: Type::Void,
                    body: vec![Statement::If(IfStatement {
                        condition: Expression::Binary(BinaryExpression {
                            lhs: Box::new(Expression::Variable(Variable {
                                name: "req".to_string(),
                                properties: vec!["http".to_string(), "host".to_string()],
                                sub_field: None
                            })),
                            operator: BinaryOperator::Eq,
                            rhs: Box::new(Expression::Literal(Literal::String(
                                "www.example.com".to_string()
                            )))
                        }),
                        body: vec![Statement::Set(SetStatement {
                            target: Variable {
                                name: "req".to_string(),
                                properties: vec!["backend_hint".to_string()],
                                sub_field: None
                            },
                            operator: SetOperator::Set,
                            value: Expression::Variable(Variable {
                                name: "example_com".to_string(),
                                properties: vec![],
                                sub_field: None
                            })
                        })],
                        els: None
                    })]
                }),]
            }
        );
        assert_eq!(
            parse(
                "sub vcl_recv {
                    if (req.http.host == \"www.example.com\") {}
                    elif (req.http.host ~ \".*\\.example\\.com\") {}
                    elsif (req.http.host == \"example.com\") {}
                    elseif (req.http.host !~ \"example\\.com\") {}
                    else if (req.http.host != \"example.com\") {}
                    else {}
                }"
            )
            .unwrap(),
            Vcl {
                declarations: vec![Declaration::Subroutine(SubroutineDeclaration {
                    name: "vcl_recv".to_string(),
                    return_type: Type::Void,
                    body: vec![Statement::If(IfStatement {
                        // if (req.http.host == \"www.example.com\") {}
                        condition: Expression::Binary(BinaryExpression {
                            lhs: Box::new(Expression::Variable(Variable {
                                name: "req".to_string(),
                                properties: vec!["http".to_string(), "host".to_string()],
                                sub_field: None
                            })),
                            operator: BinaryOperator::Eq,
                            rhs: Box::new(Expression::Literal(Literal::String(
                                "www.example.com".to_string()
                            )))
                        }),
                        body: vec![],
                        els: Some(ElseStatement::If(Box::new(IfStatement {
                            // elif (req.http.host ~ \".*\\.example\\.com\") {}
                            condition: Expression::Binary(BinaryExpression {
                                lhs: Box::new(Expression::Variable(Variable {
                                    name: "req".to_string(),
                                    properties: vec!["http".to_string(), "host".to_string()],
                                    sub_field: None
                                })),
                                operator: BinaryOperator::Tilde,
                                rhs: Box::new(Expression::Literal(Literal::String(
                                    ".*\\.example\\.com".to_string()
                                )))
                            }),
                            body: vec![],
                            els: Some(ElseStatement::If(Box::new(IfStatement {
                                // elsif (req.http.host == \"example.com\") {}
                                condition: Expression::Binary(BinaryExpression {
                                    lhs: Box::new(Expression::Variable(Variable {
                                        name: "req".to_string(),
                                        properties: vec!["http".to_string(), "host".to_string()],
                                        sub_field: None
                                    })),
                                    operator: BinaryOperator::Eq,
                                    rhs: Box::new(Expression::Literal(Literal::String(
                                        "example.com".to_string()
                                    )))
                                }),
                                body: vec![],
                                els: Some(ElseStatement::If(Box::new(IfStatement {
                                    // elseif (req.http.host !~ \"example\\.com\") {}
                                    condition: Expression::Binary(BinaryExpression {
                                        lhs: Box::new(Expression::Variable(Variable {
                                            name: "req".to_string(),
                                            properties: vec![
                                                "http".to_string(),
                                                "host".to_string()
                                            ],
                                            sub_field: None
                                        })),
                                        operator: BinaryOperator::NotTilde,
                                        rhs: Box::new(Expression::Literal(Literal::String(
                                            "example\\.com".to_string()
                                        )))
                                    }),
                                    body: vec![],
                                    els: Some(ElseStatement::If(Box::new(IfStatement {
                                        // else if (req.http.host != \"example.com\") {}
                                        condition: Expression::Binary(BinaryExpression {
                                            lhs: Box::new(Expression::Variable(Variable {
                                                name: "req".to_string(),
                                                properties: vec![
                                                    "http".to_string(),
                                                    "host".to_string()
                                                ],
                                                sub_field: None
                                            })),
                                            operator: BinaryOperator::Ne,
                                            rhs: Box::new(Expression::Literal(Literal::String(
                                                "example.com".to_string()
                                            )))
                                        }),
                                        body: vec![],
                                        els: Some(ElseStatement::Body(vec![]))
                                    })))
                                })))
                            })))
                        })))
                    })]
                })]
            }
        );
        assert_eq!(
            parse(
                "sub vcl_recv {
                    declare local var.test INTEGER;
                    set var.test = 0;
                    set var.test += 2;
                    set var.test -= 1;
                    set var.test *= 4;
                    set var.test /= 2;
                    set var.test %= 3;
                    set var.test |= 1;
                    set var.test &= 1;
                    set var.test ^= 2;
                    set var.test <<= 1;
                    set var.test >>= 1;
                    set var.test ror= 1;
                    set var.test rol= 1;
                    set var.test &&= 1;
                    set var.test ||= 1;
                    unset var.test;
                }"
            )
            .unwrap(),
            Vcl {
                declarations: vec![Declaration::Subroutine(SubroutineDeclaration {
                    name: "vcl_recv".to_string(),
                    return_type: Type::Void,
                    body: vec![
                        Statement::Declare(DeclareStatement {
                            target: Variable {
                                name: "var".to_string(),
                                properties: vec!["test".to_string()],
                                sub_field: None
                            },
                            typ: Type::Integer
                        }),
                        Statement::Set(SetStatement {
                            target: Variable {
                                name: "var".to_string(),
                                properties: vec!["test".to_string()],
                                sub_field: None
                            },
                            operator: SetOperator::Set,
                            value: Expression::Literal(Literal::Integer(0))
                        }),
                        Statement::Set(SetStatement {
                            target: Variable {
                                name: "var".to_string(),
                                properties: vec!["test".to_string()],
                                sub_field: None
                            },
                            operator: SetOperator::Add,
                            value: Expression::Literal(Literal::Integer(2))
                        }),
                        Statement::Set(SetStatement {
                            target: Variable {
                                name: "var".to_string(),
                                properties: vec!["test".to_string()],
                                sub_field: None
                            },
                            operator: SetOperator::Sub,
                            value: Expression::Literal(Literal::Integer(1))
                        }),
                        Statement::Set(SetStatement {
                            target: Variable {
                                name: "var".to_string(),
                                properties: vec!["test".to_string()],
                                sub_field: None
                            },
                            operator: SetOperator::Mul,
                            value: Expression::Literal(Literal::Integer(4))
                        }),
                        Statement::Set(SetStatement {
                            target: Variable {
                                name: "var".to_string(),
                                properties: vec!["test".to_string()],
                                sub_field: None
                            },
                            operator: SetOperator::Div,
                            value: Expression::Literal(Literal::Integer(2))
                        }),
                        Statement::Set(SetStatement {
                            target: Variable {
                                name: "var".to_string(),
                                properties: vec!["test".to_string()],
                                sub_field: None
                            },
                            operator: SetOperator::Mod,
                            value: Expression::Literal(Literal::Integer(3))
                        }),
                        Statement::Set(SetStatement {
                            target: Variable {
                                name: "var".to_string(),
                                properties: vec!["test".to_string()],
                                sub_field: None
                            },
                            operator: SetOperator::Bar,
                            value: Expression::Literal(Literal::Integer(1))
                        }),
                        Statement::Set(SetStatement {
                            target: Variable {
                                name: "var".to_string(),
                                properties: vec!["test".to_string()],
                                sub_field: None
                            },
                            operator: SetOperator::Amp,
                            value: Expression::Literal(Literal::Integer(1))
                        }),
                        Statement::Set(SetStatement {
                            target: Variable {
                                name: "var".to_string(),
                                properties: vec!["test".to_string()],
                                sub_field: None
                            },
                            operator: SetOperator::Hat,
                            value: Expression::Literal(Literal::Integer(2))
                        }),
                        Statement::Set(SetStatement {
                            target: Variable {
                                name: "var".to_string(),
                                properties: vec!["test".to_string()],
                                sub_field: None
                            },
                            operator: SetOperator::LShift,
                            value: Expression::Literal(Literal::Integer(1))
                        }),
                        Statement::Set(SetStatement {
                            target: Variable {
                                name: "var".to_string(),
                                properties: vec!["test".to_string()],
                                sub_field: None
                            },
                            operator: SetOperator::RShift,
                            value: Expression::Literal(Literal::Integer(1))
                        }),
                        Statement::Set(SetStatement {
                            target: Variable {
                                name: "var".to_string(),
                                properties: vec!["test".to_string()],
                                sub_field: None
                            },
                            operator: SetOperator::Ror,
                            value: Expression::Literal(Literal::Integer(1))
                        }),
                        Statement::Set(SetStatement {
                            target: Variable {
                                name: "var".to_string(),
                                properties: vec!["test".to_string()],
                                sub_field: None
                            },
                            operator: SetOperator::Rol,
                            value: Expression::Literal(Literal::Integer(1))
                        }),
                        Statement::Set(SetStatement {
                            target: Variable {
                                name: "var".to_string(),
                                properties: vec!["test".to_string(),],
                                sub_field: None
                            },
                            operator: SetOperator::AmpAmp,
                            value: Expression::Literal(Literal::Integer(1))
                        }),
                        Statement::Set(SetStatement {
                            target: Variable {
                                name: "var".to_string(),
                                properties: vec!["test".to_string(),],
                                sub_field: None
                            },
                            operator: SetOperator::BarBar,
                            value: Expression::Literal(Literal::Integer(1))
                        }),
                        Statement::Unset(UnsetStatement {
                            target: Variable {
                                name: "var".to_string(),
                                properties: vec!["test".to_string(),],
                                sub_field: None
                            },
                        })
                    ]
                })]
            }
        );
        assert_eq!(
            parse(
                "sub vcl_recv {
                    esi;
                    call foo;
                    error 503 \"foo\";
                    error bar;
                    error;
                }"
            )
            .unwrap(),
            Vcl {
                declarations: vec![Declaration::Subroutine(SubroutineDeclaration {
                    name: "vcl_recv".to_string(),
                    return_type: Type::Void,
                    body: vec![
                        Statement::Esi(EsiStatement),
                        Statement::Call(CallStatement {
                            target: Variable {
                                name: "foo".to_string(),
                                properties: vec![],
                                sub_field: None
                            }
                        }),
                        Statement::Error(ErrorStatement {
                            status: Some(Expression::Literal(Literal::Integer(503))),
                            message: Some(Expression::Literal(Literal::String("foo".to_string())))
                        }),
                        Statement::Error(ErrorStatement {
                            status: Some(Expression::Variable(Variable {
                                name: "bar".to_string(),
                                properties: vec![],
                                sub_field: None
                            })),
                            message: None
                        }),
                        Statement::Error(ErrorStatement {
                            status: None,
                            message: None
                        })
                    ]
                })]
            }
        );

        // sub vcl_recv {
        //     call redirect;
        //     declare local var.foo STRING;
        //     set var.foo = fun(var.count);

        //     include \"bar.vcl\";

        //     if (var.foo == \"foo\" && var.foo != \"bar\" || var.foo ~ \"^foo\") {
        //         error 503 \"foo\";
        //     }
        //     log var.foo \"bar\" + \"baz\";

        //     if (req.url ~ \"^/foo\") {
        //         restart;
        //     } else if (req.url !~ \"^/bar\") {
        //         return (pass);
        //     }

        //     esi;
        //     return (pass);
        // }
    }
}
