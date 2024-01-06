use std::vec;

use pest::iterators::Pair;
use vcls_ast::{AclDeclaration, AclEntry, Declaration};

use crate::{
    error::ParseError,
    utils::{remove_quotes, skip_comments},
    ParseResult, Rule,
};

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
        .filter_map(|p: Pair<'_, Rule>| match p.as_rule() {
            Rule::AclEntryValue => match handle_acl_entry(p) {
                Ok(entry) => Some(entry),
                Err(e) => {
                    errors.extend(e);
                    None
                }
            },
            _ => None,
        })
        .collect();
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(Declaration::Acl(AclDeclaration { name, entries }))
}

#[derive(Debug)]
struct AclEntryBuilder {
    nagated: bool,
    addr: Option<String>,
    cidr: u8,
}
impl AclEntryBuilder {
    fn new() -> Self {
        Self {
            nagated: false,
            addr: None,
            cidr: 0,
        }
    }
    fn negated(&mut self) {
        self.nagated = true;
    }
    fn addr(&mut self, addr: String) {
        self.addr = Some(addr);
    }
    fn cidr(&mut self, cidr: u8) {
        self.cidr = cidr;
    }
}
impl TryInto<AclEntry> for AclEntryBuilder {
    type Error = ParseError;
    fn try_into(self) -> Result<AclEntry, Self::Error> {
        if let Some(addr) = self.addr {
            Ok(AclEntry {
                negated: self.nagated,
                addr,
                cidr: self.cidr,
            })
        } else {
            Err(ParseError {
                message: "ACL entry address not found".to_string(),
            })
        }
    }
}

fn handle_acl_entry(pair: Pair<Rule>) -> ParseResult<AclEntry> {
    let mut builder = AclEntryBuilder::new();
    for pair in skip_comments(pair.into_inner()) {
        match pair.as_rule() {
            Rule::AclEntryNegated => builder.negated(),
            Rule::QuotedString => builder.addr(remove_quotes(pair.as_str())),
            Rule::AclEntryCidrValue => builder.cidr(pair.as_str().parse().unwrap()),
            _ => unreachable!(),
        }
    }
    builder.try_into().map_err(|e| vec![e])
}
