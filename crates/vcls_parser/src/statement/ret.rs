use pest::iterators::Pair;
use vcls_ast::ReturnStatement;

use crate::{expression, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<ReturnStatement> {
    debug_assert!(pair.as_rule() == Rule::ReturnStatement);
    let inner = pair.into_inner();
    for pair in inner {
        match pair.as_rule() {
            Rule::Expr => {
                return Ok(ReturnStatement {
                    value: Some(expression::handle(pair)?),
                })
            }
            Rule::COMMENT => {}
            _ => unreachable!("Unexpected token: {:?}", pair.as_rule()),
        }
    }
    Ok(ReturnStatement { value: None })
}
