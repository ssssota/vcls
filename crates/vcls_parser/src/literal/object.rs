use pest::iterators::Pair;
use vcls_ast::{BoolLiteral, Literal, Object, ObjectValue, RTimeLiteral};

use crate::{
    error::ParseError,
    literal::{bool, number, object, rtime, string},
    utils::convert_span,
    ParseResult, Rule,
};

pub fn handle(pair: Pair<Rule>) -> ParseResult<Object> {
    debug_assert!(pair.as_rule() == Rule::Object);
    let span = convert_span(pair.as_span());
    let inner = pair.into_inner();
    let mut entries = vec![];
    let mut errors = vec![];
    for pair in inner {
        match pair.as_rule() {
            Rule::ObjectEntry => match handle_object_entry(pair) {
                Ok(e) => entries.push(e),
                Err(e) => errors.extend(e),
            },
            Rule::COMMENT => {}
            Rule::EOI => {}
            _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }
    if errors.is_empty() {
        Ok(Object { entries, span })
    } else {
        Err(errors)
    }
}

pub fn handle_object_entry(pair: Pair<Rule>) -> ParseResult<(String, ObjectValue)> {
    debug_assert!(pair.as_rule() == Rule::ObjectEntry);
    let mut inner = pair.into_inner();
    let key = inner
        .find(|p| p.as_rule() == Rule::ObjectKey)
        .ok_or(vec![ParseError {
            message: "Object entry must have a key".to_string(),
        }])?
        .as_str();
    let value = inner
        .find(|p| {
            let rule = p.as_rule();
            rule == Rule::Object
                || rule == Rule::String
                || rule == Rule::Number
                || rule == Rule::Bool
                || rule == Rule::RTime
                || rule == Rule::Ident
        })
        .ok_or(vec![ParseError {
            message: "Object entry must have a value".to_string(),
        }])?;
    let span = convert_span(value.as_span());
    let value = match value.as_rule() {
        Rule::Object => ObjectValue::Literal(Literal::Object(object::handle(value)?)),
        Rule::String => ObjectValue::Literal(Literal::String(string::handle(value)?)),
        Rule::Number => ObjectValue::Literal(number::handle(value)?),
        Rule::Bool => ObjectValue::Literal(Literal::Bool(BoolLiteral {
            value: bool::handle(value)?,
            span,
        })),
        Rule::RTime => ObjectValue::Literal(Literal::RTime(RTimeLiteral {
            value: rtime::handle(value)?,
            span,
        })),
        Rule::Ident => ObjectValue::Ident(value.as_str().to_string()),
        _ => unreachable!("Unexpected rule: {:?}", value.as_rule()),
    };
    Ok((key[1..].to_string(), value))
}
