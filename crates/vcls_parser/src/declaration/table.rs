use pest::iterators::Pair;
use vcls_ast::{TableDeclaration, TableEntry, TableValue, Type, Variable};

use crate::{
    error::ParseError,
    literal,
    literal::string,
    utils::{convert_span, skip_comments},
    ParseResult, Rule,
};

pub fn handle(pair: Pair<Rule>) -> ParseResult<TableDeclaration> {
    debug_assert!(pair.as_rule() == Rule::TableDeclaration);
    let span = convert_span(pair.as_span());
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
        Type::from_keyword(next.as_str())
    } else {
        Type::String
    };
    match handle_table_body(next) {
        Ok(entries) => Ok(TableDeclaration {
            name,
            typ,
            entries,
            span,
        }),
        Err(e) => Err(e),
    }
}

fn handle_table_body(pair: Pair<Rule>) -> ParseResult<Vec<TableEntry>> {
    debug_assert!(pair.as_rule() == Rule::TableBody);
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
    debug_assert!(pair.as_rule() == Rule::TableEntry);
    let span = convert_span(pair.as_span());
    let mut inner = pair.into_inner();
    let key = string::handle(
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
            Rule::String | Rule::RTime | Rule::Number | Rule::Bool => {
                TableValue::Literal(literal::handle(value)?)
            }
            Rule::Ident => TableValue::Ident(Variable {
                name: value.as_str().to_string(),
                properties: vec![],
                sub_field: None,
                span: convert_span(value.as_span()),
            }),
            _ => unreachable!("Unexpected token: {:?}", value.as_rule()),
        },
        span,
    })
}
