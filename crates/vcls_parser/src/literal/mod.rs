use pest::iterators::Pair;
use vcls_ast::{BoolLiteral, Literal, RTimeLiteral};

use crate::{utils::convert_span, ParseResult, Rule};
pub mod bool;
pub mod number;
pub mod object;
pub mod rtime;
pub mod string;

pub fn handle(pair: Pair<Rule>) -> ParseResult<Literal> {
    let span = convert_span(pair.as_span());
    match pair.as_rule() {
        Rule::Literal => handle(
            pair.into_inner()
                .find(|p| p.as_rule() != Rule::COMMENT)
                .unwrap(),
        ),
        Rule::String => string::handle(pair).map(Literal::String),
        Rule::Object => object::handle(pair).map(Literal::Object),
        Rule::RTime => rtime::handle(pair).map(|r| Literal::RTime(RTimeLiteral { value: r, span })),
        Rule::Number => number::handle(pair),
        Rule::Bool => bool::handle(pair).map(|b| Literal::Bool(BoolLiteral { value: b, span })),
        _ => Err(vec![]),
    }
}
