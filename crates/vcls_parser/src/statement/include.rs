use pest::iterators::Pair;
use vcls_ast::IncludeStatement;

use crate::{
    error::ParseError,
    utils::{convert_span, remove_quotes},
    ParseResult, Rule,
};

pub fn handle(pair: Pair<Rule>) -> ParseResult<IncludeStatement> {
    debug_assert!(pair.as_rule() == Rule::IncludeStatement);
    let span = convert_span(pair.as_span());
    let mut inner = pair
        .into_inner()
        .find(|p| p.as_rule() == Rule::IncludeDeclaration)
        .unwrap()
        .into_inner();
    let quoted_path = inner
        .find(|p| p.as_rule() == Rule::QuotedString)
        .ok_or(vec![ParseError {
            message: "Include declaration must have a path".to_string(),
        }])?
        .as_str();
    let path = remove_quotes(quoted_path);
    Ok(IncludeStatement { path, span })
}
