use pest::iterators::Pair;
use vcls_ast::LogStatement;

use crate::{expression, utils::convert_span, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<LogStatement> {
    debug_assert!(pair.as_rule() == Rule::LogStatement);
    let span = convert_span(pair.as_span());
    let mut inner = pair.into_inner();
    let message = expression::handle(inner.find(|p| p.as_rule() == Rule::Expr).unwrap())?;
    Ok(LogStatement { message, span })
}
