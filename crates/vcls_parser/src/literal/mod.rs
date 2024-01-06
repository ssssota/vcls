use pest::iterators::Pair;
use vcls_ast::Literal;

use crate::{ParseResult, Rule};
pub mod bool;
pub mod number;
pub mod object;
pub mod rtime;
pub mod string;

pub fn handle_literal(pair: Pair<Rule>) -> ParseResult<Literal> {
    match pair.as_rule() {
        Rule::Literal => handle_literal(
            pair.into_inner()
                .find(|p| p.as_rule() != Rule::COMMENT)
                .unwrap(),
        ),
        Rule::String => string::handle_string(pair).map(|s| Literal::String(s)),
        Rule::Object => object::handle_object(pair),
        Rule::RTime => rtime::handle_rtime(pair),
        Rule::Number => number::handle_number(pair),
        Rule::Bool => bool::handle_bool(pair).map(|b| Literal::Bool(b)),
        _ => Err(vec![]),
    }
}
