use pest::iterators::Pair;
use vcls_ast::{EsiStatement, RestartStatement, Statement};

use crate::{ParseResult, Rule};

mod add;
mod call;
mod declare;
mod error;
mod ifs;
mod include;
mod log;
mod ret;
mod set;
mod synthetic;
mod unset;

pub fn handle(pair: Pair<Rule>) -> ParseResult<Statement> {
    debug_assert!(pair.as_rule() == Rule::Statement);
    let inner = pair.into_inner();
    for pair in inner {
        match pair.as_rule() {
            Rule::IfStatement => return ifs::handle(pair).map(Statement::If),
            Rule::SetStatement => return set::handle(pair).map(Statement::Set),
            Rule::UnsetStatement => return unset::handle(pair).map(Statement::Unset),
            Rule::AddStatement => return add::handle(pair).map(Statement::Add),
            Rule::CallStatement => return call::handle(pair).map(Statement::Call),
            Rule::DeclareStatement => return declare::handle(pair).map(Statement::Declare),
            Rule::ErrorStatement => return error::handle(pair).map(Statement::Error),
            Rule::EsiStatement => return Ok(Statement::Esi(EsiStatement)),
            Rule::IncludeStatement => return include::handle(pair).map(Statement::Include),
            Rule::LogStatement => return log::handle(pair).map(Statement::Log),
            Rule::RestartStatement => return Ok(Statement::Restart(RestartStatement)),
            Rule::ReturnStatement => return ret::handle(pair).map(Statement::Return),
            Rule::SyntheticStatement | Rule::SyntheticBase64Statement => {
                return synthetic::handle(pair).map(Statement::Synthetic)
            }
            Rule::COMMENT => {}
            _ => unreachable!("Unexpected token: {:?}", pair.as_rule()),
        }
    }
    Err(vec![]) // unreachable!()
}
