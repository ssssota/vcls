use vcls_ast::*;
use vcls_parser::parse;

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
