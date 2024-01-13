use pest::iterators::Pair;
use vcls_ast::{ElseStatement, IfStatement, Statement};

use crate::{expression, statement, utils::convert_span, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<IfStatement> {
    debug_assert!(pair.as_rule() == Rule::IfStatement || pair.as_rule() == Rule::ElseIf);
    let span = convert_span(pair.as_span());
    let mut inner = pair.into_inner();
    let condition = expression::handle(inner.find(|p| p.as_rule() == Rule::Expr).unwrap())?;
    let body = handle_body(inner.find(|p| p.as_rule() == Rule::IfBody).unwrap())?;
    let els = inner
        .find(|p| p.as_rule() == Rule::IfElse)
        .map(handle_else)
        .unwrap_or(Ok(None))?;
    Ok(IfStatement {
        condition,
        body,
        els,
        span,
    })
}

fn handle_else(pair: Pair<Rule>) -> ParseResult<Option<ElseStatement>> {
    let inner = pair.into_inner();
    for pair in inner {
        match pair.as_rule() {
            Rule::Else => {
                return Ok(Some(ElseStatement::Body(handle_body(
                    pair.into_inner()
                        .find(|p| p.as_rule() == Rule::IfBody)
                        .unwrap(),
                )?)));
            }
            Rule::ElseIf => {
                return Ok(Some(ElseStatement::If(Box::new(handle(pair)?))));
            }
            Rule::COMMENT => {}
            _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }
    Ok(None)
}

fn handle_body(pair: Pair<Rule>) -> ParseResult<Vec<Statement>> {
    debug_assert!(pair.as_rule() == Rule::IfBody);
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
    Ok(statements)
}
