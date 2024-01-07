use pest::iterators::Pair;

use crate::{ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<String> {
    if pair.as_rule() != Rule::String {
        unreachable!()
    }
    let inner = pair.into_inner();
    let mut tokens = vec![];
    for pair in inner {
        match pair.as_rule() {
            Rule::StringToken => tokens.push(handle_string_token(pair)?),
            Rule::COMMENT => {}
            _ => unreachable!("Unexpected token: {}", pair.as_str()),
        }
    }
    Ok(tokens.join(""))
}

fn handle_string_token(pair: Pair<Rule>) -> ParseResult<String> {
    if pair.as_rule() != Rule::StringToken {
        unreachable!()
    }
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::EmptyString => Ok("".to_string()),
        Rule::QuotedString => {
            let quoted = inner.as_str();
            Ok(quoted[1..quoted.len() - 1].to_string())
        }
        Rule::HeredocString | Rule::BracesQuotedString => {
            let quoted = inner.as_str();
            let quote_len = quoted.find('"').unwrap() + 1;
            Ok(quoted[quote_len..quoted.len() - quote_len].to_string())
        }
        _ => {
            println!("{:?}", inner);
            Ok("".to_string())
        } // _ => unreachable!("Unexpected token: {}", inner.as_str()),
    }
}
