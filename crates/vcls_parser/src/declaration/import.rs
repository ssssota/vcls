use pest::iterators::Pair;
use vcls_ast::ImportDeclaration;

use crate::{error::ParseError, utils::convert_span, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<ImportDeclaration> {
    let span = convert_span(pair.as_span());
    let mut inner = pair.into_inner();
    let ident = inner
        .find(|p| p.as_rule() == Rule::Ident)
        .ok_or(vec![ParseError {
            message: "Import declaration must have a identifier".to_string(),
        }])?
        .as_str()
        .to_string();
    Ok(ImportDeclaration { ident, span })
}
