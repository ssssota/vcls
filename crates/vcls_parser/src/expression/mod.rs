use std::sync::OnceLock;

use pest::{
    iterators::Pair,
    pratt_parser::{Assoc, Op, PrattParser},
};
use vcls_ast::{
    BinaryExpression, BinaryOperator, Expression, Literal, UnaryExpression, UnaryOperator,
};

use crate::{literal, variable, ParseResult, Rule};

pub fn handle(pair: Pair<Rule>) -> ParseResult<Expression> {
    debug_assert!(pair.as_rule() == Rule::Expr);
    static PARSER: OnceLock<PrattParser<Rule>> = OnceLock::new();
    let pratt = PARSER.get_or_init(|| {
        PrattParser::new()
            .op(Op::prefix(Rule::OpNot))
            .op(Op::infix(Rule::OpAnd, Assoc::Left) | Op::infix(Rule::OpOr, Assoc::Left))
            .op(Op::infix(Rule::OpEq, Assoc::Left)
                | Op::infix(Rule::OpNe, Assoc::Left)
                | Op::infix(Rule::OpLt, Assoc::Left)
                | Op::infix(Rule::OpGt, Assoc::Left)
                | Op::infix(Rule::OpLe, Assoc::Left)
                | Op::infix(Rule::OpGe, Assoc::Left))
            .op(Op::infix(Rule::OpRegexMatch, Assoc::Left)
                | Op::infix(Rule::OpRegexNotMatch, Assoc::Left))
            .op(Op::infix(Rule::OpAdd, Assoc::Left) | Op::infix(Rule::OpSub, Assoc::Left))
            .op(Op::infix(Rule::OpMul, Assoc::Left) | Op::infix(Rule::OpDiv, Assoc::Left))
            .op(Op::prefix(Rule::OpMinus))
    });
    pratt
        .map_primary(|p| match p.as_rule() {
            Rule::Primary => handle_primary(p),
            _ => unreachable!("Unexpected token: {:?}", p.as_str()),
        })
        .map_prefix(|p, rhs| match p.as_rule() {
            Rule::OpNot => Ok(Expression::Unary(UnaryExpression {
                operator: UnaryOperator::Not,
                rhs: Box::new(rhs?),
            })),
            Rule::OpMinus => Ok(Expression::Unary(UnaryExpression {
                operator: UnaryOperator::Neg,
                rhs: Box::new(rhs?),
            })),
            _ => unimplemented!("Not implemented: {:?}", p.as_rule()),
        })
        .map_infix(|lhs, p, rhs| match p.as_rule() {
            Rule::OpEq => Ok(Expression::Binary(BinaryExpression {
                lhs: Box::new(lhs?),
                operator: BinaryOperator::Eq,
                rhs: Box::new(rhs?),
            })),
            Rule::OpNe => Ok(Expression::Binary(BinaryExpression {
                lhs: Box::new(lhs?),
                operator: BinaryOperator::Ne,
                rhs: Box::new(rhs?),
            })),
            Rule::OpRegexMatch => Ok(Expression::Binary(BinaryExpression {
                lhs: Box::new(lhs?),
                operator: BinaryOperator::Tilde,
                rhs: Box::new(rhs?),
            })),
            Rule::OpRegexNotMatch => Ok(Expression::Binary(BinaryExpression {
                lhs: Box::new(lhs?),
                operator: BinaryOperator::NotTilde,
                rhs: Box::new(rhs?),
            })),
            _ => unimplemented!("Not implemented: {:?}", p.as_rule()),
        })
        .parse(pair.into_inner())
}

fn handle_primary(pair: Pair<Rule>) -> ParseResult<Expression> {
    debug_assert!(pair.as_rule() == Rule::Primary);
    let inner = pair.into_inner();
    for pair in inner {
        match pair.as_rule() {
            Rule::Literal => return literal::handle(pair).map(Expression::Literal),
            Rule::Concat => return handle_concat(pair),
            Rule::Expr => return handle(pair),
            _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }
    unreachable!("No primary found")
}

fn handle_concat(pair: Pair<Rule>) -> ParseResult<Expression> {
    debug_assert!(pair.as_rule() == Rule::Concat);
    let inner = pair.into_inner();
    let mut tokens = vec![];
    let mut errors = vec![];
    for pair in inner {
        match pair.as_rule() {
            Rule::String => match literal::string::handle(pair) {
                Ok(s) => tokens.push(Expression::Literal(Literal::String(s))),
                Err(e) => errors.extend(e),
            },
            Rule::Variable => match variable::handle(pair) {
                Ok(v) => tokens.push(Expression::Variable(v)),
                Err(e) => errors.extend(e),
            },
            _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }
    if errors.is_empty() {
        Ok(fold_concat(&tokens))
    } else {
        Err(errors)
    }
}

fn fold_concat(tokens: &[Expression]) -> Expression {
    if tokens.len() == 1 {
        tokens[0].clone()
    } else {
        Expression::Binary(BinaryExpression {
            lhs: Box::new(tokens[0].clone()),
            operator: BinaryOperator::Add,
            rhs: Box::new(fold_concat(&tokens[1..])),
        })
    }
}
