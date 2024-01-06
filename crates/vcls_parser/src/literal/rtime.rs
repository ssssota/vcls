use pest::iterators::Pair;
use vcls_ast::{Literal, RelativeTime};

use crate::{error::ParseError, ParseResult, Rule};

pub fn handle_rtime(pair: Pair<Rule>) -> ParseResult<Literal> {
    if pair.as_rule() != Rule::RTime {
        return Err(vec![]);
    }
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
        "ms" => Ok(Literal::RTime(RelativeTime::from_ms(value as u64))),
        "s" => Ok(Literal::RTime(RelativeTime::from_sec(value))),
        "m" => Ok(Literal::RTime(RelativeTime::from_min(value))),
        "h" => Ok(Literal::RTime(RelativeTime::from_hour(value))),
        "d" => Ok(Literal::RTime(RelativeTime::from_day(value))),
        "y" => Ok(Literal::RTime(RelativeTime::from_year(value))),
        _ => unreachable!("Unexpected unit: {}", unit),
    }
}
