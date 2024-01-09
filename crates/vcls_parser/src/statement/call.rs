use pest::iterators::Pair;
use vcls_ast::{CallStatement, Variable};

use crate::{ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<CallStatement> {
    debug_assert!(pair.as_rule() == Rule::CallStatement);
    let mut inner = pair.into_inner();
    let target = inner
        .find(|p| p.as_rule() == Rule::Ident)
        .unwrap()
        .as_str()
        .to_string();
    Ok(CallStatement {
        target: Variable {
            name: target,
            properties: vec![],
            sub_field: None,
        },
    })
}
