use error::ParseError;
use pest::Parser;
use vcls_ast::*;

mod declaration;
mod error;
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
            Rule::EOI => {}
            _ => unimplemented!(),
        }
    }
    Ok(Vcl { declarations })
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

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
    \"localhost\";
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
                            addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
                            cidr: 0,
                        },
                        AclEntry {
                            negated: false,
                            addr: IpAddr::V4(Ipv4Addr::new(192, 0, 2, 0)),
                            cidr: 24,
                        },
                        AclEntry {
                            negated: true,
                            addr: IpAddr::V4(Ipv4Addr::new(192, 0, 2, 12)),
                            cidr: 0,
                        },
                        AclEntry {
                            negated: false,
                            addr: IpAddr::V4(Ipv4Addr::new(198, 51, 100, 4)),
                            cidr: 0,
                        },
                        AclEntry {
                            negated: false,
                            addr: IpAddr::V6(Ipv6Addr::new(
                                0x2001, 0xdb8, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff
                            )),
                            cidr: 0,
                        }
                    ],
                })]
            }
        );
    }
}
