use vcls_ast::*;
use vcls_parser::parse;

#[test]
fn blank() {
    assert_eq!(
        parse("acl foo {}").unwrap(),
        Vcl {
            declarations: vec![Declaration::Acl(AclDeclaration {
                name: "foo".to_string(),
                entries: vec![],
                span: Span(0, 10),
            })],
        },
    );
}

#[test]
fn localhost() {
    assert_eq!(
        parse(r#"acl foo { "localhost"; }"#).unwrap(),
        Vcl {
            declarations: vec![Declaration::Acl(AclDeclaration {
                name: "foo".to_string(),
                entries: vec![AclEntry {
                    negated: false,
                    addr: "localhost".to_string(),
                    cidr: 0,
                    span: Span(10, 21),
                }],
                span: Span(0, 24),
            })],
        },
    );
}

#[test]
fn ipv4() {
    assert_eq!(
        parse(r#"acl foo { "192.0.2.0"; }"#).unwrap(),
        Vcl {
            declarations: vec![Declaration::Acl(AclDeclaration {
                name: "foo".to_string(),
                entries: vec![AclEntry {
                    negated: false,
                    addr: "192.0.2.0".to_string(),
                    cidr: 0,
                    span: Span(10, 21),
                }],
                span: Span(0, 24),
            })],
        },
    );
}

#[test]
fn ipv6() {
    assert_eq!(
        parse(r#"acl foo { "2001:db8:ffff:ffff:ffff:ffff:ffff:ffff"; }"#).unwrap(),
        Vcl {
            declarations: vec![Declaration::Acl(AclDeclaration {
                name: "foo".to_string(),
                entries: vec![AclEntry {
                    negated: false,
                    addr: "2001:db8:ffff:ffff:ffff:ffff:ffff:ffff".to_string(),
                    cidr: 0,
                    span: Span(10, 50),
                }],
                span: Span(0, 53),
            })],
        },
    );
}

#[test]
fn cidr() {
    assert_eq!(
        parse(
            r#"acl foo {
                "1920.2.0"/24;
                "2001:db8:ffff:ffff:ffff:ffff:ffff:ffff"/64;
            }"#
        )
        .unwrap(),
        Vcl {
            declarations: vec![Declaration::Acl(AclDeclaration {
                name: "foo".to_string(),
                entries: vec![
                    AclEntry {
                        negated: false,
                        addr: "1920.2.0".to_string(),
                        cidr: 24,
                        span: Span(26, 39),
                    },
                    AclEntry {
                        negated: false,
                        addr: "2001:db8:ffff:ffff:ffff:ffff:ffff:ffff".to_string(),
                        cidr: 64,
                        span: Span(57, 100),
                    }
                ],
                span: Span(0, 115),
            })],
        },
    );
}

#[test]
fn negation() {
    assert_eq!(
        parse(
            r#"acl foo {
                ! "1920.2.0"/24;
                ! "2001:db8:ffff:ffff:ffff:ffff:ffff:ffff"/64;
            }"#
        )
        .unwrap(),
        Vcl {
            declarations: vec![Declaration::Acl(AclDeclaration {
                name: "foo".to_string(),
                entries: vec![
                    AclEntry {
                        negated: true,
                        addr: "1920.2.0".to_string(),
                        cidr: 24,
                        span: Span(26, 41),
                    },
                    AclEntry {
                        negated: true,
                        addr: "2001:db8:ffff:ffff:ffff:ffff:ffff:ffff".to_string(),
                        cidr: 64,
                        span: Span(59, 104),
                    }
                ],
                span: Span(0, 119),
            })],
        },
    );
}

#[test]
fn acl() {
    assert_eq!(
        parse(
            r#"
acl office_ip_ranges {
    "localhost";                                 # loopback
    "192.0.2.0"/24;                              # internal office...
    ! "192.0.2.12";                              # ... except for the vending machine
    "198.51.100.4";                              # remote VPN office
    "2001:db8:ffff:ffff:ffff:ffff:ffff:ffff";    # ipv6 address remote
}"#,
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
                        span: Span(28, 39),
                    },
                    AclEntry {
                        negated: false,
                        addr: "192.0.2.0".to_string(),
                        cidr: 24,
                        span: Span(88, 102),
                    },
                    AclEntry {
                        negated: true,
                        addr: "192.0.2.12".to_string(),
                        cidr: 0,
                        span: Span(158, 172),
                    },
                    AclEntry {
                        negated: false,
                        addr: "198.51.100.4".to_string(),
                        cidr: 0,
                        span: Span(244, 258),
                    },
                    AclEntry {
                        negated: false,
                        addr: "2001:db8:ffff:ffff:ffff:ffff:ffff:ffff".to_string(),
                        cidr: 0,
                        span: Span(313, 353),
                    }
                ],
                span: Span(1, 381)
            })]
        }
    );
}
