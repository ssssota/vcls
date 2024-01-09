use pest::iterators::Pair;
use vcls_ast::LogStatement;

use crate::{expression, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<LogStatement> {
    debug_assert!(pair.as_rule() == Rule::LogStatement);
    let mut inner = pair.into_inner();
    let message = expression::handle(inner.find(|p| p.as_rule() == Rule::Expr).unwrap())?;
    Ok(LogStatement { message })
}
