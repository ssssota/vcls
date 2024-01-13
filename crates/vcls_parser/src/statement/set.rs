use std::str::FromStr;

use pest::iterators::Pair;
use vcls_ast::{SetOperator, SetStatement};

use crate::{error::ParseError, expression, utils::convert_span, variable, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<SetStatement> {
    debug_assert!(pair.as_rule() == Rule::SetStatement);
    let span = convert_span(pair.as_span());
    let mut inner = pair.into_inner();
    let target = variable::handle(inner.find(|p| p.as_rule() == Rule::Variable).unwrap())?;
    let operator = SetOperator::from_str(
        inner
            .find(|p| p.as_rule() == Rule::SetOperator)
            .unwrap()
            .as_str(),
    )
    .map_err(|_| {
        vec![ParseError {
            message: format!("Invalid set operator: {:?}", inner.as_str()),
        }]
    })?;
    let value = expression::handle(inner.find(|p| p.as_rule() == Rule::Expr).unwrap())?;
    Ok(SetStatement {
        target,
        operator,
        value,
        span,
    })
}
