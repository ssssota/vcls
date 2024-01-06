use pest::iterators::Pair;
use vcls_ast::Literal;

use crate::{error::ParseError, ParseResult, Rule};

use super::handle_literal;

pub fn handle_object(pair: Pair<Rule>) -> ParseResult<Literal> {
    if pair.as_rule() != Rule::Object {
        return Err(vec![]);
    }
    let inner = pair.into_inner();
    let mut obj = vec![];
    let mut errors = vec![];
    for pair in inner {
        match pair.as_rule() {
            Rule::ObjectEntry => match handle_object_entry(pair) {
                Ok(e) => obj.push(e),
                Err(e) => errors.extend(e),
            },
            Rule::EOI => {}
            _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }
    if errors.is_empty() {
        Ok(Literal::Object(obj))
    } else {
        Err(errors)
    }
}

pub fn handle_object_entry(pair: Pair<Rule>) -> ParseResult<(String, Literal)> {
    if pair.as_rule() != Rule::ObjectEntry {
        return Err(vec![]);
    }
    let mut inner = pair.into_inner();
    let key = inner
        .find(|p| p.as_rule() == Rule::ObjectKey)
        .ok_or(vec![ParseError {
            message: "Object entry must have a key".to_string(),
        }])?
        .as_str()
        .to_string();
    let value = inner
        .find(|p| p.as_rule() == Rule::ObjectValue)
        .ok_or(vec![ParseError {
            message: "Object entry must have a value".to_string(),
        }])?;
    match handle_literal(value) {
        Ok(lit) => Ok((key, lit)),
        Err(e) => Err(e),
    }
}
