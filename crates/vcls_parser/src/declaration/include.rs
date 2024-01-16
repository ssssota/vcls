use pest::iterators::Pair;
use vcls_ast::IncludeDeclaration;

use crate::{
    error::ParseError, literal::string::handle_quoted_string, utils::convert_span, ParseResult,
    Rule,
};

pub fn handle(pair: Pair<Rule>) -> ParseResult<IncludeDeclaration> {
    let span = convert_span(pair.as_span());
    let mut inner = pair.into_inner();
    let quoted_string = inner
        .find(|p| p.as_rule() == Rule::QuotedString)
        .ok_or(vec![ParseError {
            message: "Include declaration must have a path".to_string(),
        }])?;
    Ok(IncludeDeclaration {
        path: handle_quoted_string(quoted_string),
        span,
    })
}
