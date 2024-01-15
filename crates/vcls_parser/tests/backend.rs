use vcls_ast::*;
use vcls_parser::parse;

#[test]
fn blank() {
    assert_eq!(
        parse("backend foo {}").unwrap(),
        Vcl {
            declarations: vec![Declaration::Backend(BackendDeclaration {
                name: "foo".to_string(),
                config: Object {
                    entries: vec![],
                    span: Span(12, 14)
                },
                span: Span(0, 14),
            })],
        },
    );
}

#[test]
fn host() {
    assert_eq!(
        parse(r#"backend foo { .host = "example.com"; }"#).unwrap(),
        Vcl {
            declarations: vec![Declaration::Backend(BackendDeclaration {
                name: "foo".to_string(),
                config: Object {
                    entries: vec![(
                        "host".to_string(),
                        ObjectValue::Literal(Literal::String(StringLiteral {
                            value: "example.com".to_string(),
                            span: Span(22, 35),
                        })),
                    )],
                    span: Span(12, 38),
                },
                span: Span(0, 38),
            })],
        },
    );
}

#[test]
fn backend() {
    assert_eq!(
        parse(
            r#"
backend backend_name {
    # Required to be set for all VCL defined backends
    .dynamic = true;

    # Server location
    .host = "storage.googleapis.com";
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
        .request = "HEAD / HTTP/1.1"  "Host: storage.googleapis.com" "Connection: close";
        .expected_response = 200;
        .interval = 60s;   # Send a check every 60s
        .timeout = 2s;     # Allow up to 2s for the backend to respond to the check
        .window = 5;       # Keep a history of 5 checks
        .initial = 4;      # Start with 4 successful checks in the history
        .threshold = 4;    # 4 of the recent checks must be successful for backend to be healthy
    }
}"#
        )
        .unwrap(),
        Vcl {
            declarations: vec![Declaration::Backend(BackendDeclaration {
                name: "backend_name".to_string(),
                config: Object {
                    entries: vec![
                        (
                            "dynamic".to_string(),
                            ObjectValue::Literal(Literal::Bool(BoolLiteral{value:true,span:Span(93,97)})),
                        ),
                        (
                            "host".to_string(),
                            ObjectValue::Literal(Literal::String(StringLiteral {
                                value:"storage.googleapis.com".to_string(),
                                span: Span(134, 158),
                            })),
                        ),
                        ("ssl".to_string(), ObjectValue::Literal(Literal::Bool(BoolLiteral{value:true,span:Span(171, 175)}))),
                        (
                            "ssl_check_cert".to_string(),
                            ObjectValue::Ident("always".to_string()),
                        ),
                        (
                            "connect_timeout".to_string(),
                            ObjectValue::Literal(Literal::RTime(RTimeLiteral{value:RelativeTime::from_sec(1.0),span:Span(257,259)})),
                        ),
                        (
                            "max_connections".to_string(),
                            ObjectValue::Literal(Literal::Integer(IntegerLiteral{value:200,span:Span(284,287)})),
                        ),
                        (
                            "probe".to_string(),
                            ObjectValue::Literal(
                                Literal::Object(Object { entries: vec![
                                    (
                                        "dummy".to_string(),
                                        ObjectValue::Literal(Literal::Bool(BoolLiteral{value:false,span:Span(341,346)})),
                                    ),
                                    (
                                        "request".to_string(),
                                        ObjectValue::Literal(Literal::String(StringLiteral{
                                            value:"HEAD / HTTP/1.1Host: storage.googleapis.comConnection: close"
                                                .to_string(),
                                                span:Span(560,629),
                                        })),
                                    ),
                                    (
                                        "expected_response".to_string(),
                                        ObjectValue::Literal(Literal::Integer(IntegerLiteral{value:200,span:Span(660,663)})),
                                    ),
                                    (
                                        "interval".to_string(),
                                        ObjectValue::Literal(Literal::RTime(RTimeLiteral{
                                            value:RelativeTime::from_sec(60.0),
                                            span:Span(685,688)
                                        })),
                                    ),
                                    (
                                        "timeout".to_string(),
                                        ObjectValue::Literal(Literal::RTime(RTimeLiteral{
                                            value:RelativeTime::from_sec(2.0),
                                            span:Span(736,738)
                                        })),
                                    ),
                                    (
                                        "window".to_string(),
                                        ObjectValue::Literal(Literal::Integer(IntegerLiteral{value:5,span:Span(819,820)})),
                                    ),
                                    (
                                        "initial".to_string(),
                                        ObjectValue::Literal(Literal::Integer(IntegerLiteral{value:4,span:Span(876,877)})),
                                    ),
                                    (
                                        "threshold".to_string(),
                                        ObjectValue::Literal(Literal::Integer(IntegerLiteral{value:4,span:Span(953,954)})),
                                    ),
                                ],span: Span(322, 1034), }),
                            ),
                        ),
                    ],
                    span: Span(22, 1036),
                },
                span: Span(1, 1036),
            })],
        },
    );
}
