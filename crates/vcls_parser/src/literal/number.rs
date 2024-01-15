use pest::iterators::Pair;
use vcls_ast::{FloatLiteral, IntegerLiteral, Literal};

use crate::{error::ParseError, utils::convert_span, ParseResult, Rule};

// TODO: Handle hexadecimals
pub fn handle(pair: Pair<Rule>) -> ParseResult<Literal> {
    debug_assert!(pair.as_rule() == Rule::Number);
    let span = convert_span(pair.as_span());
    let num = pair.as_str();
    if num.contains('.') {
        Ok(Literal::Float(FloatLiteral {
            value: num.parse().map_err(|e| {
                vec![ParseError {
                    message: format!("Failed to parse float: {}", e),
                }]
            })?,
            span,
        }))
    } else {
        Ok(Literal::Integer(IntegerLiteral {
            value: num.parse().map_err(|e| {
                vec![ParseError {
                    message: format!("Failed to parse integer: {}", e),
                }]
            })?,
            span,
        }))
    }
}
