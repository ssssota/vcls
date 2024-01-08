use pest::iterators::Pair;
use vcls_ast::Literal;

use crate::{error::ParseError, ParseResult, Rule};

// TODO: Handle hexadecimals
pub fn handle(pair: Pair<Rule>) -> ParseResult<Literal> {
    debug_assert!(pair.as_rule() == Rule::Number);
    let num = pair.as_str();
    if num.contains('.') {
        Ok(Literal::Float(num.parse().map_err(|e| {
            vec![ParseError {
                message: format!("Failed to parse float: {}", e),
            }]
        })?))
    } else {
        Ok(Literal::Integer(num.parse().map_err(|e| {
            vec![ParseError {
                message: format!("Failed to parse integer: {}", e),
            }]
        })?))
    }
}
