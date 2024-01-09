use pest::iterators::Pair;
use vcls_ast::Declaration;

use crate::{ParseResult, Rule};

pub mod acl;
pub mod backend;
pub mod import;
pub mod include;
pub mod penaltybox;
pub mod ratecounter;
pub mod sub;
pub mod table;

pub fn handle(pair: Pair<Rule>) -> ParseResult<Declaration> {
    debug_assert!(pair.as_rule() == Rule::Declaration);
    let inner = pair.into_inner();
    for pair in inner {
        match pair.as_rule() {
            Rule::IncludeDeclaration => return Ok(Declaration::Include(include::handle(pair)?)),
            Rule::ImportDeclaration => return Ok(Declaration::Import(import::handle(pair)?)),
            Rule::AclDeclaration => return Ok(Declaration::Acl(acl::handle(pair)?)),
            Rule::PenaltyboxDeclaration => {
                return Ok(Declaration::PenaltyBox(penaltybox::handle(pair)?))
            }
            Rule::RateCounterDeclaration => {
                return Ok(Declaration::RateCounter(ratecounter::handle(pair)?))
            }
            Rule::TableDeclaration => return Ok(Declaration::Table(table::handle(pair)?)),
            Rule::BackendDeclaration => return Ok(Declaration::Backend(backend::handle(pair)?)),
            Rule::SubDeclaration => return Ok(Declaration::Subroutine(sub::handle(pair)?)),
            Rule::COMMENT => {}
            _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }
    unreachable!()
}
