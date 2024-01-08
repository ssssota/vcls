use pest::iterators::Pair;
use vcls_ast::RelativeTime;

use crate::{error::ParseError, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<RelativeTime> {
    debug_assert!(pair.as_rule() == Rule::RTime);
    let mut inner = pair.into_inner();
    let value = inner
        .find(|p| p.as_rule() == Rule::RTimeValue)
        .ok_or(vec![ParseError {
            message: "RTime must have a value".to_string(),
        }])?
        .as_str()
        .parse()
        .map_err(|e| {
            vec![ParseError {
                message: format!("Failed to parse RTime value: {}", e).to_string(),
            }]
        })?;
    let unit = inner
        .find(|p| p.as_rule() == Rule::RTimeUnit)
        .ok_or(vec![ParseError {
            message: "RTime must have a unit".to_string(),
        }])?
        .as_str();
    match unit {
        "ms" => Ok(RelativeTime::from_ms(value as u64)),
        "s" => Ok(RelativeTime::from_sec(value)),
        "m" => Ok(RelativeTime::from_min(value)),
        "h" => Ok(RelativeTime::from_hour(value)),
        "d" => Ok(RelativeTime::from_day(value)),
        "y" => Ok(RelativeTime::from_year(value)),
        _ => unreachable!("Unexpected unit: {}", unit),
    }
}
