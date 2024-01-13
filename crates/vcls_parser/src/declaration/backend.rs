use pest::iterators::Pair;
use vcls_ast::BackendDeclaration;

use crate::{literal::object, utils::convert_span, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<BackendDeclaration> {
    let span = convert_span(pair.as_span());
    let mut inner = pair.into_inner();
    let name = inner
        .find(|p| p.as_rule() == Rule::Ident)
        .unwrap()
        .as_str()
        .to_string();
    let obj = inner.find(|p| p.as_rule() == Rule::Object).unwrap();
    Ok(BackendDeclaration {
        name,
        config: object::handle(obj)?,
        span,
    })
}
