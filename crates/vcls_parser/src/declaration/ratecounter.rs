use pest::iterators::Pair;
use vcls_ast::{Declaration, RateCounterDeclaration};

use crate::{error::ParseError, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<Declaration> {
    let mut inner = pair.into_inner();
    let name = inner
        .find(|p| p.as_rule() == Rule::Ident)
        .ok_or(vec![ParseError {
            message: "Import declaration must have a identifier".to_string(),
        }])?
        .as_str()
        .to_string();
    Ok(Declaration::RateCounter(RateCounterDeclaration { name }))
}
