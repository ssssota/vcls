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
        Rule::String => string::handle(pair).map(Literal::String),
        Rule::Object => object::handle(pair).map(Literal::Object),
        Rule::RTime => rtime::handle(pair).map(Literal::RTime),
        Rule::Number => number::handle(pair),
        Rule::Bool => bool::handle(pair).map(Literal::Bool),
        _ => Err(vec![]),
    }
}
