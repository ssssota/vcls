use error::ParseError;
use pest::Parser;
use vcls_ast::*;

mod declaration;
mod error;
mod literal;
mod utils;

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
  .share_key = \"YOUR_SERVICE_ID\";

  # Server location
  .host = \"storage.googleapis.com\";
  .port = \"443\";
  .ssl = true;
  .ssl_cert_hostname = \"storage.googleapis.com\";
  .ssl_check_cert = always;
  .ssl_sni_hostname = \"storage.googleapis.com\";

  # Timeouts and limits
  .between_bytes_timeout = 10s;
  .connect_timeout = 1s;
  .first_byte_timeout = 15s;
  .max_connections = 200;

  # Host header override
  .host_header = \"storage.googleapis.com\";
  .always_use_host_header = true;

  # Protected properties
  .bypass_local_route_table = true;

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
                    config: Some(vec![
                        ("dynamic".to_string(), Literal::Bool(true)),
                        (
                            "share_key".to_string(),
                            Literal::String("YOUR_SERVICE_ID".to_string())
                        ),
                        (
                            "host".to_string(),
                            Literal::String("storage.googleapis.com".to_string())
                        ),
                        ("port".to_string(), Literal::String("443".to_string())),
                        ("ssl".to_string(), Literal::Bool(true)),
                        (
                            "ssl_cert_hostname".to_string(),
                            Literal::String("storage.googleapis.com".to_string())
                        ),
                        (
                            "ssl_check_cert".to_string(),
                            (Variable {
                                name: "always".to_string(),
                                properties: vec![],
                                sub_field: None
                            })
                        )
                    ])
                })]
            }
        );
    }
}
