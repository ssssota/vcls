use pest::iterators::Pair;
use vcls_ast::{CallStatement, Variable};

use crate::{utils::convert_span, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<CallStatement> {
    debug_assert!(pair.as_rule() == Rule::CallStatement);
    let span = convert_span(pair.as_span());
    let mut inner = pair.into_inner();
    let target = inner.find(|p| p.as_rule() == Rule::Ident).unwrap();
    Ok(CallStatement {
        target: Variable {
            name: target.as_str().to_string(),
            properties: vec![],
            sub_field: None,
            span: convert_span(target.as_span()),
        },
        span,
    })
}
