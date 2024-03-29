use pest::iterators::Pair;
use vcls_ast::{DeclareStatement, Type};

use crate::{utils::convert_span, variable, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<DeclareStatement> {
    debug_assert!(pair.as_rule() == Rule::DeclareStatement);
    let span = convert_span(pair.as_span());
    let mut inner = pair.into_inner();
    let target = variable::handle(inner.find(|p| p.as_rule() == Rule::Variable).unwrap())?;
    let typ = Type::from_keyword(inner.find(|p| p.as_rule() == Rule::Type).unwrap().as_str());
    Ok(DeclareStatement { target, typ, span })
}
