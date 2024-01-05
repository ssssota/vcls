use std::vec;

use pest::iterators::Pair;
use vcls_ast::{AclDeclaration, AclEntry, Declaration};

use crate::{error::ParseError, utils::skip_comments, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<Declaration> {
    let mut errors = vec![];
    let mut inner = skip_comments(pair.into_inner());
    let name = match inner.find(|p| p.as_rule() == Rule::Ident) {
        Some(p) => p.as_str(),
        None => {
            errors.push(ParseError {
                message: "ACL name not found".to_string(),
            });
            ""
        }
    }
    .to_string();
    let entries: Vec<AclEntry> = inner
        .filter_map(|p| match p.as_rule() {
            Rule::AclEntry => {
                println!("ACL entry: {:?}", p);
                Some(AclEntry::Ipv4 {
                    addr: [0, 0, 0, 0],
                    cidr: 0,
                    negated: false,
                })
            }
            _ => None,
        })
        .collect();
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(Declaration::Acl(AclDeclaration { name, entries }))
}
