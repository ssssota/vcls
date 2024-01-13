use pest::iterators::Pair;
use vcls_ast::ErrorStatement;

use crate::{expression, utils::convert_span, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<ErrorStatement> {
    debug_assert!(pair.as_rule() == Rule::ErrorStatement);
    let span = convert_span(pair.as_span());
    let mut inner = pair.into_inner();
    let info = inner.find(|p| p.as_rule() == Rule::ErrorInfo);
    if let Some(p) = info {
        let mut inner = p.into_inner();
        let status = Some(expression::handle(
            inner.find(|p| p.as_rule() == Rule::Expr).unwrap(),
        )?);
        let message = inner
            .find(|p| p.as_rule() == Rule::Expr)
            .map(|p| expression::handle(p))
            .transpose()?;
        Ok(ErrorStatement {
            status,
            message,
            span,
        })
    } else {
        Ok(ErrorStatement {
            status: None,
            message: None,
            span,
        })
    }
}
