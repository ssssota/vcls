use pest::iterators::Pair;
use vcls_ast::AddStatement;

use crate::{expression, utils::convert_span, variable, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<AddStatement> {
    debug_assert!(pair.as_rule() == Rule::AddStatement);
    let span = convert_span(pair.as_span());
    let mut inner = pair.into_inner();
    let target = variable::handle(inner.find(|p| p.as_rule() == Rule::Variable).unwrap())?;
    let value = expression::handle(inner.find(|p| p.as_rule() == Rule::Expr).unwrap())?;
    Ok(AddStatement {
        target,
        value,
        span,
    })
}
