use pest::iterators::Pair;
use vcls_ast::SyntheticStatement;

use crate::{expression, utils::convert_span, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<SyntheticStatement> {
    let rule = pair.as_rule();
    debug_assert!(rule == Rule::SyntheticStatement || rule == Rule::SyntheticBase64Statement);
    let span = convert_span(pair.as_span());
    let base64 = rule == Rule::SyntheticBase64Statement;
    let value = expression::handle(
        pair.into_inner()
            .find(|p| p.as_rule() == Rule::Expr)
            .unwrap(),
    )?;
    Ok(SyntheticStatement {
        value,
        base64,
        span,
    })
}
