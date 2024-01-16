use pest::iterators::Pair;
use vcls_ast::{StringLiteral, StringToken};

use crate::{utils::convert_span, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<StringLiteral> {
    debug_assert!(pair.as_rule() == Rule::String);
    let span = convert_span(pair.as_span());
    let inner = pair.into_inner();
    let mut tokens = vec![];
    let mut errors = vec![];
    for pair in inner {
        match pair.as_rule() {
            Rule::StringToken => match handle_string_token(pair) {
                Ok(token) => tokens.push(token),
                Err(err) => errors.extend(err),
            },
            Rule::COMMENT => {}
            _ => unreachable!("Unexpected token: {}", pair.as_str()),
        }
    }
    if errors.is_empty() {
        Ok(StringLiteral { tokens, span })
    } else {
        Err(errors)
    }
}

fn handle_string_token(pair: Pair<Rule>) -> ParseResult<StringToken> {
    debug_assert!(pair.as_rule() == Rule::StringToken);
    let span = convert_span(pair.as_span());
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::EmptyString => Ok(StringToken {
            value: "".to_string(),
            span,
        }),
        Rule::QuotedString => {
            let quoted = inner.as_str();
            Ok(StringToken {
                value: quoted[1..quoted.len() - 1].to_string(),
                span,
            })
        }
        Rule::HeredocString | Rule::BracesQuotedString => {
            let quoted = inner.as_str();
            let quote_len = quoted.find('"').unwrap() + 1;
            Ok(StringToken {
                value: quoted[quote_len..quoted.len() - quote_len].to_string(),
                span,
            })
        }
        _ => unreachable!("Unexpected token: {}", inner.as_str()),
    }
}

pub fn handle_quoted_string(pair: Pair<Rule>) -> StringToken {
    debug_assert!(pair.as_rule() == Rule::QuotedString);
    let span = convert_span(pair.as_span());
    let quoted = pair.as_str();
    StringToken {
        value: quoted[1..quoted.len() - 1].to_string(),
        span,
    }
}
