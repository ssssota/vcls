use pest::iterators::Pair;
use vcls_ast::PenaltyBoxDeclaration;

use crate::{error::ParseError, utils::convert_span, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<PenaltyBoxDeclaration> {
    let span = convert_span(pair.as_span());
    let mut inner = pair.into_inner();
    let name = inner
        .find(|p| p.as_rule() == Rule::Ident)
        .ok_or(vec![ParseError {
            message: "Import declaration must have a identifier".to_string(),
        }])?
        .as_str()
        .to_string();
    Ok(PenaltyBoxDeclaration { name, span })
}
