use pest::iterators::Pair;
use vcls_ast::{EsiStatement, RestartStatement, Statement};

use crate::{ParseResult, Rule};

mod call;
mod declare;
mod error;
mod ifs;
mod ret;
mod set;
mod synthetic;
mod unset;

pub fn handle(pair: Pair<Rule>) -> ParseResult<Statement> {
    debug_assert!(pair.as_rule() == Rule::Statement);
    let inner = pair.into_inner();
    for pair in inner {
        match pair.as_rule() {
            Rule::IfStatement => {
                return Ok(ifs::handle(pair).map(|s| Statement::If(s))?);
            }
            Rule::SetStatement => {
                return Ok(set::handle(pair).map(|s| Statement::Set(s))?);
            }
            Rule::UnsetStatement => {
                return Ok(unset::handle(pair).map(|s| Statement::Unset(s))?);
            }
            Rule::AddStatement => {}
            Rule::CallStatement => {
                return Ok(call::handle(pair).map(|s| Statement::Call(s))?);
            }
            Rule::DeclareStatement => {
                return Ok(declare::handle(pair).map(|s| Statement::Declare(s))?);
            }
            Rule::ErrorStatement => {
                return Ok(error::handle(pair).map(|s| Statement::Error(s))?);
            }
            Rule::EsiStatement => {
                return Ok(Statement::Esi(EsiStatement));
            }
            Rule::IncludeStatement => {}
            Rule::LogStatement => {}
            Rule::RestartStatement => {
                return Ok(Statement::Restart(RestartStatement));
            }
            Rule::ReturnStatement => {
                return Ok(ret::handle(pair).map(|s| Statement::Return(s))?);
            }
            Rule::SyntheticStatement | Rule::SyntheticBase64Statement => {
                return Ok(synthetic::handle(pair).map(|s| Statement::Synthetic(s))?)
            }
            Rule::COMMENT => {}
            _ => unreachable!("Unexpected token: {:?}", pair.as_rule()),
        }
    }
    Err(vec![]) // unreachable!()
}
