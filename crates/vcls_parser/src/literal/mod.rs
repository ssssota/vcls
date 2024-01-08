use pest::iterators::Pair;
use vcls_ast::Literal;

use crate::{ParseResult, Rule};
pub mod bool;
pub mod number;
pub mod object;
pub mod rtime;
pub mod string;

pub fn handle(pair: Pair<Rule>) -> ParseResult<Literal> {
    match pair.as_rule() {
        Rule::Literal => handle(
            pair.into_inner()
                .find(|p| p.as_rule() != Rule::COMMENT)
                .unwrap(),
        ),
        Rule::String => string::handle(pair).map(|s| Literal::String(s)),
        Rule::Object => object::handle(pair).map(|o| Literal::Object(o)),
        Rule::RTime => rtime::handle(pair).map(|r| Literal::RTime(r)),
        Rule::Number => number::handle(pair),
        Rule::Bool => bool::handle(pair).map(|b| Literal::Bool(b)),
        _ => Err(vec![]),
    }
}
