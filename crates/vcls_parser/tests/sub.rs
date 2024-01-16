use vcls_ast::*;
use vcls_parser::parse;

#[test]
fn ret() {
    assert_eq!(
        parse("sub foo { return /* noop */; }").unwrap(),
        Vcl {
            declarations: vec![Declaration::Subroutine(SubroutineDeclaration {
                name: "foo".to_string(),
                return_type: Type::Void,
                body: vec![Statement::Return(ReturnStatement {
                    value: None,
                    span: Span(10, 28),
                })],
                span: Span(8, 30),
            })],
        },
    );
}

#[test]
fn ret_value() {
    assert_eq!(
        parse(r#"sub foo STRING { return "foo" with_variable.bar:baz; }"#).unwrap(),
        Vcl {
            declarations: vec![Declaration::Subroutine(SubroutineDeclaration {
                name: "foo".to_string(),
                return_type: Type::String,
                body: vec![Statement::Return(ReturnStatement {
                    value: Some(Expression::Binary(BinaryExpression {
                        lhs: Box::new(Expression::Literal(Literal::String(StringLiteral {
                            tokens: vec![StringToken {
                                value: "foo".to_string(),
                                span: Span(24, 29),
                            }],
                            span: Span(24, 30),
                        }))),
                        operator: BinaryOperator::Add,
                        rhs: Box::new(Expression::Variable(Variable {
                            name: "with_variable".to_string(),
                            properties: vec!["bar".to_string()],
                            sub_field: Some("baz".to_string()),
                            span: Span(30, 51),
                        })),
                        span: Span(24, 51),
                    })),
                    span: Span(24, 51),
                })],
                span: Span(15, 54),
            })],
        },
    );
}

#[test]
fn synthetic() {
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
                        value: Expression::Literal(Literal::String(StringLiteral {
                            tokens: vec![StringToken {
                                value: "dGVzdA==".to_string(),
                                span: Span(49, 59),
                            }],
                            span: Span(49, 59),
                        })),
                        span: Span(32, 60),
                    }),
                    Statement::Synthetic(SyntheticStatement {
                        base64: false,
                        value: Expression::Literal(Literal::String(StringLiteral {
                            tokens: vec![StringToken {
                                value: "foo".to_string(),
                                span: Span(87, 92)
                            }],
                            span: Span(87, 92)
                        })),
                        span: Span(77, 93),
                    }),
                ],
                span: Span(14, 107),
            })]
        },
    );
}

#[test]
fn if_set() {
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
                            sub_field: None,
                            span: Span(35, 49),
                        })),
                        operator: BinaryOperator::Eq,
                        rhs: Box::new(Expression::Literal(Literal::String(StringLiteral {
                            tokens: vec![StringToken {
                                value: "www.example.com".to_string(),
                                span: Span(52, 69)
                            }],
                            span: Span(52, 69)
                        }))),
                        span: Span(49, 51),
                    }),
                    body: vec![Statement::Set(SetStatement {
                        target: Variable {
                            name: "req".to_string(),
                            properties: vec!["backend_hint".to_string()],
                            sub_field: None,
                            span: Span(97, 114),
                        },
                        operator: SetOperator::Set,
                        value: Expression::Variable(Variable {
                            name: "example_com".to_string(),
                            properties: vec![],
                            sub_field: None,
                            span: Span(116, 127),
                        }),
                        span: Span(93, 128),
                    })],
                    els: None,
                    span: Span(31, 159),
                })],
                span: Span(13, 160),
            }),],
        },
    );
}

#[test]
fn if_else() {
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
                            sub_field: None,
                            span: Span(35, 49),
                        })),
                        operator: BinaryOperator::Eq,
                        rhs: Box::new(Expression::Literal(Literal::String(StringLiteral {
                            tokens: vec![StringToken {
                                value: "www.example.com".to_string(),
                                span: Span(52, 69),
                            }],
                            span: Span(52, 69),
                        }))),
                        span: Span(49, 51),
                    }),
                    body: vec![],
                    els: Some(ElseStatement::If(Box::new(IfStatement {
                        // elif (req.http.host ~ ".*\\.example\\.com") {}
                        condition: Expression::Binary(BinaryExpression {
                            lhs: Box::new(Expression::Variable(Variable {
                                name: "req".to_string(),
                                properties: vec!["http".to_string(), "host".to_string()],
                                sub_field: None,
                                span: Span(96, 110),
                            })),
                            operator: BinaryOperator::Tilde,
                            rhs: Box::new(Expression::Literal(Literal::String(StringLiteral {
                                tokens: vec![StringToken {
                                    value: ".*\\.example\\.com".to_string(),
                                    span: Span(112, 130),
                                }],
                                span: Span(112, 130),
                            }))),
                            span: Span(110, 111),
                        }),
                        body: vec![],
                        els: Some(ElseStatement::If(Box::new(IfStatement {
                            // elsif (req.http.host == "example.com") {}
                            condition: Expression::Binary(BinaryExpression {
                                lhs: Box::new(Expression::Variable(Variable {
                                    name: "req".to_string(),
                                    properties: vec!["http".to_string(), "host".to_string()],
                                    sub_field: None,
                                    span: Span(158, 172),
                                })),
                                operator: BinaryOperator::Eq,
                                rhs: Box::new(Expression::Literal(Literal::String(
                                    StringLiteral {
                                        tokens: vec![StringToken {
                                            value: "example.com".to_string(),
                                            span: Span(175, 188),
                                        }],
                                        span: Span(175, 188),
                                    }
                                ))),
                                span: Span(172, 174),
                            }),
                            body: vec![],
                            els: Some(ElseStatement::If(Box::new(IfStatement {
                                // elseif (req.http.host !~ "example\\.com") {}
                                condition: Expression::Binary(BinaryExpression {
                                    lhs: Box::new(Expression::Variable(Variable {
                                        name: "req".to_string(),
                                        properties: vec!["http".to_string(), "host".to_string()],
                                        sub_field: None,
                                        span: Span(217, 231),
                                    })),
                                    operator: BinaryOperator::NotTilde,
                                    rhs: Box::new(Expression::Literal(Literal::String(
                                        StringLiteral {
                                            tokens: vec![StringToken {
                                                value: "example\\.com".to_string(),
                                                span: Span(234, 248),
                                            }],
                                            span: Span(234, 248),
                                        }
                                    ))),
                                    span: Span(231, 233),
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
                                            sub_field: None,
                                            span: Span(278, 292),
                                        })),
                                        operator: BinaryOperator::Ne,
                                        rhs: Box::new(Expression::Literal(Literal::String(
                                            StringLiteral {
                                                tokens: vec![StringToken {
                                                    value: "example.com".to_string(),
                                                    span: Span(295, 308),
                                                }],
                                                span: Span(295, 308),
                                            }
                                        ))),
                                        span: Span(292, 294),
                                    }),
                                    body: vec![],
                                    els: Some(ElseStatement::Body(vec![])),
                                    span: Span(269, 336),
                                }))),
                                span: Span(209, 336),
                            }))),
                            span: Span(151, 336),
                        }))),
                        span: Span(90, 336),
                    }))),
                    span: Span(31, 336),
                })],
                span: Span(13, 350),
            })],
        },
    );
}

#[test]
fn set() {
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
                            sub_field: None,
                            span: Span(45, 54),
                        },
                        typ: Type::Integer,
                        span: Span(31, 62),
                    }),
                    Statement::Set(SetStatement {
                        target: Variable {
                            name: "var".to_string(),
                            properties: vec!["test".to_string()],
                            sub_field: None,
                            span: Span(83, 92),
                        },
                        operator: SetOperator::Set,
                        value: Expression::Literal(Literal::Integer(IntegerLiteral {
                            value: 0,
                            span: Span(94, 95)
                        })),
                        span: Span(79, 96),
                    }),
                    Statement::Set(SetStatement {
                        target: Variable {
                            name: "var".to_string(),
                            properties: vec!["test".to_string()],
                            sub_field: None,
                            span: Span(117, 126),
                        },
                        operator: SetOperator::Add,
                        value: Expression::Literal(Literal::Integer(IntegerLiteral {
                            value: 2,
                            span: Span(129, 130),
                        })),
                        span: Span(113, 131),
                    }),
                    Statement::Set(SetStatement {
                        target: Variable {
                            name: "var".to_string(),
                            properties: vec!["test".to_string()],
                            sub_field: None,
                            span: Span(152, 161),
                        },
                        operator: SetOperator::Sub,
                        value: Expression::Literal(Literal::Integer(IntegerLiteral {
                            value: 1,
                            span: Span(164, 165),
                        })),
                        span: Span(148, 166),
                    }),
                    Statement::Set(SetStatement {
                        target: Variable {
                            name: "var".to_string(),
                            properties: vec!["test".to_string()],
                            sub_field: None,
                            span: Span(187, 196),
                        },
                        operator: SetOperator::Mul,
                        value: Expression::Literal(Literal::Integer(IntegerLiteral {
                            value: 4,
                            span: Span(199, 200)
                        })),
                        span: Span(183, 201),
                    }),
                    Statement::Set(SetStatement {
                        target: Variable {
                            name: "var".to_string(),
                            properties: vec!["test".to_string()],
                            sub_field: None,
                            span: Span(222, 231),
                        },
                        operator: SetOperator::Div,
                        value: Expression::Literal(Literal::Integer(IntegerLiteral {
                            value: 2,
                            span: Span(234, 235),
                        })),
                        span: Span(218, 236),
                    }),
                    Statement::Set(SetStatement {
                        target: Variable {
                            name: "var".to_string(),
                            properties: vec!["test".to_string()],
                            sub_field: None,
                            span: Span(257, 266),
                        },
                        operator: SetOperator::Mod,
                        value: Expression::Literal(Literal::Integer(IntegerLiteral {
                            value: 3,
                            span: Span(269, 270),
                        })),
                        span: Span(253, 271),
                    }),
                    Statement::Set(SetStatement {
                        target: Variable {
                            name: "var".to_string(),
                            properties: vec!["test".to_string()],
                            sub_field: None,
                            span: Span(292, 301),
                        },
                        operator: SetOperator::Bar,
                        value: Expression::Literal(Literal::Integer(IntegerLiteral {
                            value: 1,
                            span: Span(304, 305),
                        })),
                        span: Span(288, 306),
                    }),
                    Statement::Set(SetStatement {
                        target: Variable {
                            name: "var".to_string(),
                            properties: vec!["test".to_string()],
                            sub_field: None,
                            span: Span(327, 336),
                        },
                        operator: SetOperator::Amp,
                        value: Expression::Literal(Literal::Integer(IntegerLiteral {
                            value: 1,
                            span: Span(339, 340),
                        })),
                        span: Span(323, 341),
                    }),
                    Statement::Set(SetStatement {
                        target: Variable {
                            name: "var".to_string(),
                            properties: vec!["test".to_string()],
                            sub_field: None,
                            span: Span(362, 371),
                        },
                        operator: SetOperator::Hat,
                        value: Expression::Literal(Literal::Integer(IntegerLiteral {
                            value: 2,
                            span: Span(374, 375),
                        })),
                        span: Span(358, 376),
                    }),
                    Statement::Set(SetStatement {
                        target: Variable {
                            name: "var".to_string(),
                            properties: vec!["test".to_string()],
                            sub_field: None,
                            span: Span(397, 406),
                        },
                        operator: SetOperator::LShift,
                        value: Expression::Literal(Literal::Integer(IntegerLiteral {
                            value: 1,
                            span: Span(410, 411),
                        })),
                        span: Span(393, 412),
                    }),
                    Statement::Set(SetStatement {
                        target: Variable {
                            name: "var".to_string(),
                            properties: vec!["test".to_string()],
                            sub_field: None,
                            span: Span(433, 442),
                        },
                        operator: SetOperator::RShift,
                        value: Expression::Literal(Literal::Integer(IntegerLiteral {
                            value: 1,
                            span: Span(446, 447),
                        })),
                        span: Span(429, 448),
                    }),
                    Statement::Set(SetStatement {
                        target: Variable {
                            name: "var".to_string(),
                            properties: vec!["test".to_string()],
                            sub_field: None,
                            span: Span(469, 478),
                        },
                        operator: SetOperator::Ror,
                        value: Expression::Literal(Literal::Integer(IntegerLiteral {
                            value: 1,
                            span: Span(483, 484),
                        })),
                        span: Span(465, 485),
                    }),
                    Statement::Set(SetStatement {
                        target: Variable {
                            name: "var".to_string(),
                            properties: vec!["test".to_string()],
                            sub_field: None,
                            span: Span(506, 515),
                        },
                        operator: SetOperator::Rol,
                        value: Expression::Literal(Literal::Integer(IntegerLiteral {
                            value: 1,
                            span: Span(520, 521),
                        })),
                        span: Span(502, 522),
                    }),
                    Statement::Set(SetStatement {
                        target: Variable {
                            name: "var".to_string(),
                            properties: vec!["test".to_string(),],
                            sub_field: None,
                            span: Span(543, 552),
                        },
                        operator: SetOperator::AmpAmp,
                        value: Expression::Literal(Literal::Integer(IntegerLiteral {
                            value: 1,
                            span: Span(556, 557),
                        })),
                        span: Span(539, 558),
                    }),
                    Statement::Set(SetStatement {
                        target: Variable {
                            name: "var".to_string(),
                            properties: vec!["test".to_string(),],
                            sub_field: None,
                            span: Span(579, 588),
                        },
                        operator: SetOperator::BarBar,
                        value: Expression::Literal(Literal::Integer(IntegerLiteral {
                            value: 1,
                            span: Span(592, 593),
                        })),
                        span: Span(575, 594),
                    }),
                    Statement::Unset(UnsetStatement {
                        target: Variable {
                            name: "var".to_string(),
                            properties: vec!["test".to_string(),],
                            sub_field: None,
                            span: Span(617, 625),
                        },
                        span: Span(611, 626),
                    }),
                ],
                span: Span(13, 640),
            })],
        },
    );
}

#[test]
fn error() {
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
                    Statement::Esi(EsiStatement { span: Span(31, 35) }),
                    Statement::Call(CallStatement {
                        target: Variable {
                            name: "foo".to_string(),
                            properties: vec![],
                            sub_field: None,
                            span: Span(57, 60),
                        },
                        span: Span(52, 61),
                    }),
                    Statement::Error(ErrorStatement {
                        status: Some(Expression::Literal(Literal::Integer(IntegerLiteral {
                            value: 503,
                            span: Span(84, 87)
                        }))),
                        message: Some(Expression::Literal(Literal::String(StringLiteral {
                            tokens: vec![StringToken {
                                value: "foo".to_string(),
                                span: Span(88, 93)
                            }],
                            span: Span(88, 93)
                        }))),
                        span: Span(78, 94),
                    }),
                    Statement::Error(ErrorStatement {
                        status: Some(Expression::Variable(Variable {
                            name: "bar".to_string(),
                            properties: vec![],
                            sub_field: None,
                            span: Span(117, 120),
                        })),
                        message: None,
                        span: Span(111, 121),
                    }),
                    Statement::Error(ErrorStatement {
                        status: None,
                        message: None,
                        span: Span(138, 144),
                    }),
                    Statement::Restart(RestartStatement {
                        span: Span(161, 169)
                    }),
                ],
                span: Span(13, 183),
            })],
        },
    );
}

#[test]
fn include() {
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
                    path: StringToken {
                        value: "foo.vcl".to_string(),
                        span: Span(39, 48),
                    },
                    span: Span(31, 49),
                })],
                span: Span(13, 63),
            })],
        },
    );
}

#[test]
fn log() {
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
                        message: Expression::Literal(Literal::String(
                            StringLiteral{
                                tokens: vec![
                                    StringToken { value: "foo".to_string(), span: Span(38,43) }
                                ],
                                span: Span(38,43),
                            }
                        )),
                        span: Span(34, 44),
                    }),
                    Statement::Add(AddStatement {
                        target: Variable {
                            name: "resp".to_string(),
                            properties: vec!["http".to_string(), "Set-Cookie".to_string()],
                            sub_field: None,
                            span: Span(65,86),
                        },
                        value: Expression::Literal(Literal::String(StringLiteral {
                            tokens: vec![StringToken { value: "myCookie=foo; path=/; SameSite=Strict; Secure; max-age=60".to_string(), span: Span(88, 147),}],
                            span: Span(88, 147),
                        })),
                        span: Span(61,148),
                    }),
                ],
                span: Span(16,162),
            })],
        },
    )
}
