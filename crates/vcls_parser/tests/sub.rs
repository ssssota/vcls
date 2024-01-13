use vcls_ast::*;
use vcls_parser::parse;

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
        parse(r#"sub foo STRING { return "foo" with_variable.bar:baz; }"#).unwrap(),
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
            r#"sub vcl_error {
                synthetic.base64 "dGVzdA==";
                synthetic "foo";
            }"#
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
            r#"sub vcl_recv {
                if (req.http.host == "www.example.com") {
                    set req.backend_hint = example_com;
                }
            }"#
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
            r#"sub vcl_recv {
                if (req.http.host == "www.example.com") {}
                elif (req.http.host ~ ".*\.example\.com") {}
                elsif (req.http.host == "example.com") {}
                elseif (req.http.host !~ "example\.com") {}
                else if (req.http.host != "example.com") {}
                else {}
            }"#
        )
        .unwrap(),
        Vcl {
            declarations: vec![Declaration::Subroutine(SubroutineDeclaration {
                name: "vcl_recv".to_string(),
                return_type: Type::Void,
                body: vec![Statement::If(IfStatement {
                    // if (req.http.host == "www.example.com") {}
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
                        // elif (req.http.host ~ ".*\\.example\\.com") {}
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
                            // elsif (req.http.host == "example.com") {}
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
                                // elseif (req.http.host !~ "example\\.com") {}
                                condition: Expression::Binary(BinaryExpression {
                                    lhs: Box::new(Expression::Variable(Variable {
                                        name: "req".to_string(),
                                        properties: vec!["http".to_string(), "host".to_string()],
                                        sub_field: None
                                    })),
                                    operator: BinaryOperator::NotTilde,
                                    rhs: Box::new(Expression::Literal(Literal::String(
                                        "example\\.com".to_string()
                                    )))
                                }),
                                body: vec![],
                                els: Some(ElseStatement::If(Box::new(IfStatement {
                                    // else if (req.http.host != "example.com") {}
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
            r#"sub vcl_recv {
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
            }"#
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
            r#"sub vcl_recv {
                esi;
                call foo;
                error 503 "foo";
                error bar;
                error;
                restart;
            }"#
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
                    }),
                    Statement::Restart(RestartStatement),
                ]
            })]
        }
    );
    assert_eq!(
        parse(
            r#"sub vcl_recv {
                include "foo.vcl";
            }"#
        )
        .unwrap(),
        Vcl {
            declarations: vec![Declaration::Subroutine(SubroutineDeclaration {
                name: "vcl_recv".to_string(),
                return_type: Type::Void,
                body: vec![Statement::Include(IncludeStatement {
                    path: "foo.vcl".to_string()
                })]
            })]
        }
    );
    assert_eq!(
        parse(
            r#"sub vcl_deliver {
                log "foo";
                add resp.http.Set-Cookie = "myCookie=foo; path=/; SameSite=Strict; Secure; max-age=60";
            }"#
    ).unwrap(),
        Vcl {
            declarations: vec![Declaration::Subroutine(SubroutineDeclaration {
                name: "vcl_deliver".to_string(),
                return_type: Type::Void,
                body: vec![
                    Statement::Log(LogStatement {
                        message: Expression::Literal(Literal::String("foo".to_string()))
                    }),
                    Statement::Add(AddStatement {
                        target: Variable {
                            name: "resp".to_string(),
                            properties: vec!["http".to_string(), "Set-Cookie".to_string()],
                            sub_field: None
                        },
                        value: Expression::Literal(Literal::String(
                            "myCookie=foo; path=/; SameSite=Strict; Secure; max-age=60".to_string()
                        ))
                    })
                ]
            })]
        }
    )
}
