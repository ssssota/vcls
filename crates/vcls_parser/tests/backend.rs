use vcls_ast::*;
use vcls_parser::parse;

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
