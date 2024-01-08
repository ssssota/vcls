use pest::iterators::Pair;
use vcls_ast::Statement;

use crate::{ParseResult, Rule};

mod ifs;
mod ret;
mod set;
mod synthetic;

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
            Rule::UnsetStatement => {}
            Rule::AddStatement => {}
            Rule::CallStatement => {}
            Rule::DeclareStatement => {}
            Rule::ErrorStatement => {}
            Rule::EsiStatement => {}
            Rule::IncludeStatement => {}
            Rule::LogStatement => {}
            Rule::RestartStatement => {}
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
