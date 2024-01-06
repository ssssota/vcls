use pest::iterators::Pair;

use crate::{error::ParseError, ParseResult, Rule};

pub fn handle_bool(pair: Pair<Rule>) -> ParseResult<bool> {
    if pair.as_rule() != Rule::Bool {
        return Err(vec![]);
    }
    match pair.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(vec![ParseError {
            message: "Failed to parse boolean".to_string(),
        }]),
    }
}
