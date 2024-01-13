use pest::iterators::Pair;
use vcls_ast::ReturnStatement;

use crate::{expression, utils::convert_span, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<ReturnStatement> {
    debug_assert!(pair.as_rule() == Rule::ReturnStatement);
    let span = convert_span(pair.as_span());
    let inner = pair.into_inner();
    for pair in inner {
        let span = convert_span(pair.as_span());
        match pair.as_rule() {
            Rule::Expr => {
                return Ok(ReturnStatement {
                    value: Some(expression::handle(pair)?),
                    span,
                })
            }
            Rule::COMMENT => {}
            _ => unreachable!("Unexpected token: {:?}", pair.as_rule()),
        }
    }
    Ok(ReturnStatement { value: None, span })
}
