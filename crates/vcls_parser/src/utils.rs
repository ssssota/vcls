use pest::iterators::Pairs;

use crate::Rule;

pub fn skip_comments(pairs: Pairs<Rule>) -> impl Iterator<Item = pest::iterators::Pair<Rule>> {
    pairs.filter(|p| match p.as_rule() {
        Rule::COMMENT => false,
        _ => true,
    })
}
