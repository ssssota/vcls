use pest::iterators::Pair;
use vcls_ast::{Statement, SubroutineDeclaration, Type};

use crate::{error::ParseError, statement, utils::convert_span, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<SubroutineDeclaration> {
    debug_assert!(pair.as_rule() == Rule::SubDeclaration);
    let mut inner = pair.into_inner();
    let name = inner
        .find(|p| p.as_rule() == Rule::Ident)
        .unwrap()
        .as_str()
        .to_string();
    let mut typ = Type::Void;
    for pair in inner {
        let span = convert_span(pair.as_span());
        match pair.as_rule() {
            Rule::Type => {
                typ = Type::from_keyword(pair.as_str());
            }
            Rule::SubBody => {
                return Ok(SubroutineDeclaration {
                    name,
                    return_type: typ,
                    body: handle_sub_body(pair)?,
                    span,
                })
            }
            Rule::COMMENT => {}
            _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }
    Err(vec![ParseError {
        message: "Subroutine declaration must have a body".to_string(),
    }])
}

fn handle_sub_body(pair: Pair<Rule>) -> ParseResult<Vec<Statement>> {
    debug_assert!(pair.as_rule() == Rule::SubBody);
    let inner = pair.into_inner();
    let mut statements = vec![];
    let mut errors = vec![];
    for pair in inner {
        match pair.as_rule() {
            Rule::Statement => match statement::handle(pair) {
                Ok(s) => statements.push(s),
                Err(e) => errors.extend(e),
            },
            Rule::COMMENT => {}
            _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }
    if errors.is_empty() {
        Ok(statements)
    } else {
        Err(errors)
    }
}
