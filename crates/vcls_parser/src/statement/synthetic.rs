use pest::iterators::Pair;
use vcls_ast::SyntheticStatement;

use crate::{expression, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<SyntheticStatement> {
    let rule = pair.as_rule();
    if rule != Rule::SyntheticStatement && rule != Rule::SyntheticBase64Statement {
        unreachable!()
    }
    let base64 = rule == Rule::SyntheticBase64Statement;
    let value = expression::handle(
        pair.into_inner()
            .find(|p| p.as_rule() == Rule::Expr)
            .unwrap(),
    )?;
    Ok(SyntheticStatement { value, base64 })
}
