use error::ParseError;
use pest::Parser;
use vcls_ast::*;

mod declaration;
mod error;
mod expression;
mod literal;
mod statement;
mod utils;
mod variable;

#[derive(pest_derive::Parser)]
#[grammar = "./fastly-vcl.pest"]
struct VclParser;

pub type ParseResult<T> = Result<T, Vec<error::ParseError>>;

pub fn parse(src: &str) -> ParseResult<Vcl> {
    let mut errors = vec![];
    let pairs = VclParser::parse(Rule::Vcl, src).map_err(|e| {
        vec![ParseError {
            message: e.to_string(),
        }]
    })?;
    let mut declarations = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::Declaration => match declaration::handle(pair) {
                Ok(declaration) => declarations.push(declaration),
                Err(mut err) => errors.append(&mut err),
            },
            Rule::EOI => {}
            _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }
    Ok(Vcl { declarations })
}
