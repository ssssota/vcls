use pest::iterators::Pair;
use vcls_ast::Variable;

use crate::{ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<Variable> {
    debug_assert!(pair.as_rule() == Rule::Variable);
    let mut inner = pair.into_inner();
    let name = inner
        .find(|p| p.as_rule() == Rule::Ident)
        .unwrap()
        .as_str()
        .to_string();
    let mut properties = vec![];
    let mut sub_field = None;
    for pair in inner {
        match pair.as_rule() {
            Rule::VariableProp => properties.push(pair.as_str()[1..].to_string()),
            Rule::SubfieldAccess => {
                sub_field = Some(pair.as_str()[1..].to_string());
            }
            Rule::COMMENT => {}
            _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }
    Ok(Variable {
        name,
        properties,
        sub_field,
    })
}
