use pest::iterators::Pair;
use vcls_ast::{Declaration, IncludeDeclaration};

use crate::{error::ParseError, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<Declaration> {
    let mut inner = pair.into_inner();
    let quoted_path = inner
        .find(|p| p.as_rule() == Rule::QuotedString)
        .ok_or(vec![ParseError {
            message: "Include declaration must have a path".to_string(),
        }])?
        .as_str();
    let path = quoted_path[1..quoted_path.len() - 1].to_string();
    Ok(Declaration::Include(IncludeDeclaration { path }))
}
