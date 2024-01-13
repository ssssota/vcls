use pest::iterators::Pair;
use vcls_ast::UnsetStatement;

use crate::{utils::convert_span, variable, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<UnsetStatement> {
    debug_assert!(pair.as_rule() == Rule::UnsetStatement);
    let span = convert_span(pair.as_span());
    let mut inner = pair.into_inner();
    let target = variable::handle(inner.find(|p| p.as_rule() == Rule::Variable).unwrap())?;
    Ok(UnsetStatement { target, span })
}
