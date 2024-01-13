use pest::iterators::Pairs;

use crate::Rule;

pub fn skip_comments(pairs: Pairs<Rule>) -> impl Iterator<Item = pest::iterators::Pair<Rule>> {
    pairs.filter(|p| !matches!(p.as_rule(), Rule::COMMENT))
}

#[inline]
pub fn remove_quotes(s: &str) -> String {
    s[1..s.len() - 1].to_string()
}

pub fn convert_span(span: pest::Span) -> vcls_ast::Span {
    vcls_ast::Span(span.start(), span.end())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_quote() {
        assert_eq!(remove_quotes("\"foo\""), "foo".to_string());
    }
}
