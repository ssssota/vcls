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
    use super::*;

    #[test]
    fn blank() {
        assert_eq!(
            parse(""),
            Ok(Vcl {
                declarations: vec![]
            })
        );
    }

    #[test]
    fn include() {
        assert_eq!(
            parse("include \"foo.vcl\";"),
            Ok(Vcl {
                declarations: vec![Declaration::Include(IncludeDeclaration {
                    path: "foo.vcl".to_string()
                })]
            })
        );
    }

    #[test]
    fn import() {
        assert_eq!(
            parse("import foo;"),
            Ok(Vcl {
                declarations: vec![Declaration::Import(ImportDeclaration {
                    ident: "foo".to_string()
                })]
            })
        );
    }

    #[test]
    fn acl() {
        assert_eq!(
            parse(
                "\
acl office_ip_ranges {
    \"192.0.2.0\"/24;                              # internal office...
    ! \"192.0.2.12\";                              # ... except for the vending machine
    \"198.51.100.4\";                              # remote VPN office
    \"2001:db8:ffff:ffff:ffff:ffff:ffff:ffff\";    # ipv6 address remote
}"
            ),
            Ok(Vcl {
                declarations: vec![Declaration::Acl(AclDeclaration {
                    name: "office_ip_ranges".to_string(),
                    entries: vec![
                        AclEntry::Ipv4 {
                            addr: [192, 0, 2, 0],
                            cidr: 24,
                            negated: false
                        },
                        AclEntry::Ipv4 {
                            addr: [192, 0, 2, 12],
                            cidr: 0,
                            negated: true
                        },
                        AclEntry::Ipv4 {
                            addr: [198, 51, 100, 4],
                            cidr: 0,
                            negated: false
                        },
                        AclEntry::Ipv6 {
                            addr: [
                                0x20, 0x01, 0x0d, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                                0xff, 0xff, 0xff, 0xff, 0xff
                            ],
                            cidr: 0,
                            negated: false
                        }
                    ],
                })]
            })
        );
    }
}
