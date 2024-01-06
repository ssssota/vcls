use pest::iterators::Pair;
use vcls_ast::{Declaration, TableDeclaration, TableEntry, TableValue, Type, Variable};

use crate::{
    error::ParseError,
    literal::{handle_literal, string::handle_string},
    utils::skip_comments,
    ParseResult, Rule,
};

pub fn handle(pair: Pair<Rule>) -> ParseResult<Declaration> {
    let mut inner = skip_comments(pair.into_inner());
    let name = inner
        .next()
        .ok_or(vec![ParseError {
            message: "Table must have a name".to_string(),
        }])?
        .as_str()
        .to_string();
    let next = inner.next().ok_or(vec![ParseError {
        message: "Table must have a body".to_string(),
    }])?;
    let typ = if next.as_rule() == Rule::TableType {
        Type::from_str(next.as_str())
    } else {
        Type::String
    };
    match handle_table_body(next) {
        Ok(entries) => Ok(Declaration::Table(TableDeclaration { name, typ, entries })),
        Err(e) => Err(e),
    }
}

fn handle_table_body(pair: Pair<Rule>) -> ParseResult<Vec<TableEntry>> {
    if pair.as_rule() != Rule::TableBody {
        return Err(vec![ParseError {
            message: "Expected table body".to_string(),
        }]);
    }
    let mut entries = vec![];
    let mut errors = vec![];
    for entry in pair.into_inner() {
        match entry.as_rule() {
            Rule::TableEntry => match handle_table_entry(entry) {
                Ok(e) => entries.push(e),
                Err(e) => errors.extend(e),
            },
            Rule::COMMENT => {}
            _ => unreachable!("Unexpected token: {:?}", entry.as_rule()),
        }
    }
    if errors.is_empty() {
        Ok(entries)
    } else {
        Err(errors)
    }
}

fn handle_table_entry(pair: Pair<Rule>) -> ParseResult<TableEntry> {
    if pair.as_rule() != Rule::TableEntry {
        return Err(vec![]);
    }
    let mut inner = pair.into_inner();
    let key = handle_string(
        inner
            .find(|p| p.as_rule() == Rule::TableKey)
            .ok_or(vec![ParseError {
                message: "Table entry must have a key".to_string(),
            }])?
            .into_inner()
            .next()
            .unwrap(),
    )?;
    let value = inner
        .find(|p| p.as_rule() == Rule::TableValue)
        .ok_or(vec![ParseError {
            message: "Table entry must have a value".to_string(),
        }])?
        .into_inner()
        .next()
        .unwrap();
    Ok(TableEntry {
        key,
        value: match value.as_rule() {
            Rule::Literal => TableValue::Literal(handle_literal(value)?),
            Rule::Ident => TableValue::Ident(Variable {
                name: value.as_str().to_string(),
                properties: vec![],
                sub_field: None,
            }),
            _ => unreachable!("Unexpected token: {:?}", value.as_rule()),
        },
    })
}
